use crate::api::TranscriptSegment;
use crate::diarization::assign::{label_rows, RowLabel, RowSpan};
use crate::diarization::diarizer::{DiarizeOptions, Diarizer};
use crate::diarization::timing::{read_metadata, recording_time_map, TimeMap};
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

const RATE_16K: usize = 16_000;
const NATIVE_RATE: u32 = 48_000;
const AAC_FRAME: usize = 1024;

struct Args {
    folder: PathBuf,
    models: PathBuf,
    num_speakers: Option<usize>,
    preview: Option<PathBuf>,
}

fn parse_args() -> Result<Args> {
    let mut it = std::env::args().skip(1);
    let usage = "usage: eval-diarize <meeting_folder> <models_dir> [num_speakers|auto] [preview.md]";
    let folder = PathBuf::from(it.next().context(usage)?);
    let models = PathBuf::from(it.next().context(usage)?);
    let num_speakers = match it.next().as_deref() {
        None | Some("auto") => None,
        Some(n) => Some(n.parse().with_context(|| format!("bad speaker count {n}"))?),
    };
    let preview = it.next().map(PathBuf::from);
    Ok(Args { folder, models, num_speakers, preview })
}

/// Decoded frames at the native rate, counted from AAC packets (1024 samples each, priming and
/// padding included, as the app's symphonia decoder returns them).
fn native_frames(audio: &Path) -> Result<usize> {
    let out = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "a:0", "-count_packets"])
        .args(["-show_entries", "stream=nb_read_packets", "-of", "csv=p=0"])
        .arg(audio)
        .output()
        .context("ffprobe failed to start")?;
    let packets: usize = String::from_utf8_lossy(&out.stdout).trim().parse().context("ffprobe packet count")?;
    Ok(packets * AAC_FRAME)
}

/// 16 kHz mono f32 with every decoded frame kept (edit list ignored), clamped to [-1, 1].
fn decode_16k(audio: &Path) -> Result<Vec<f32>> {
    let mut child = Command::new("ffmpeg")
        .args(["-v", "fatal", "-ignore_editlist", "1", "-i"])
        .arg(audio)
        .args(["-f", "f32le", "-ac", "1", "-ar", "16000", "pipe:1"])
        .stdout(Stdio::piped())
        .spawn()
        .context("ffmpeg failed to start")?;
    let mut bytes = Vec::new();
    child.stdout.take().context("ffmpeg stdout")?.read_to_end(&mut bytes)?;
    if !child.wait()?.success() {
        bail!("ffmpeg could not decode {}", audio.display());
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]).clamp(-1.0, 1.0))
        .collect())
}

fn load_rows(folder: &Path) -> Result<Vec<TranscriptSegment>> {
    let raw = std::fs::read_to_string(folder.join("transcripts.json")).context("transcripts.json")?;
    let value: serde_json::Value = serde_json::from_str(&raw)?;
    let list = value.get("segments").cloned().unwrap_or(value);
    let mut rows: Vec<TranscriptSegment> = serde_json::from_value(list).context("transcript rows")?;
    rows.sort_by(|a, b| a.audio_start_time.partial_cmp(&b.audio_start_time).unwrap_or(std::cmp::Ordering::Equal));
    Ok(rows)
}

fn mmss(s: f64) -> String {
    let s = s.max(0.0) as u64;
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

pub fn main() -> Result<()> {
    let args = parse_args()?;
    let audio = args.folder.join("audio.mp4");
    let metadata = read_metadata(&args.folder);
    let frames = native_frames(&audio)?;
    let time_map = recording_time_map(metadata.as_ref(), NATIVE_RATE, frames);

    let t0 = Instant::now();
    let samples = decode_16k(&audio)?;
    let decode_s = t0.elapsed().as_secs_f64();
    let audio_s = samples.len() as f64 / RATE_16K as f64;

    let t1 = Instant::now();
    let mut diarizer = Diarizer::load(&args.models)?;
    let opts = DiarizeOptions { num_speakers: args.num_speakers, ..Default::default() };
    let d = diarizer.diarize(&samples, &opts, &mut |_| {}, &|| false)?;
    let diarize_s = t1.elapsed().as_secs_f64();

    let rows = load_rows(&args.folder)?;
    let spans: Vec<Option<RowSpan>> = rows
        .iter()
        .map(|r| match (r.audio_start_time, r.audio_end_time) {
            (Some(s), Some(e)) if e > s => Some(RowSpan { start_s: time_map.file_s(s), end_s: time_map.file_s(e) }),
            _ => None,
        })
        .collect();
    let labels = label_rows(&spans, &d.turns);

    let mut speech: BTreeMap<String, f64> = BTreeMap::new();
    for t in &d.turns {
        *speech.entry(t.key.clone()).or_default() += t.duration();
    }
    let (mut single, mut mixed, mut unlabeled) = (0, 0, 0);
    for l in &labels {
        match l {
            RowLabel::Single(_) => single += 1,
            RowLabel::Mixed { .. } => mixed += 1,
            RowLabel::Unlabeled => unlabeled += 1,
        }
    }

    println!("meeting   : {}", args.folder.file_name().unwrap_or_default().to_string_lossy());
    println!("time map  : {:?}", time_map);
    println!("audio     : {:.1} min, decode {:.1}s", audio_s / 60.0, decode_s);
    println!("diarize   : {:.1}s  ({:.0}x real time)", diarize_s, audio_s / diarize_s.max(1e-6));
    println!("speakers  : {} (turns {}, overlap turns {})", d.speakers.len(), d.turns.len(), d.overlap.len());
    for (key, secs) in &speech {
        println!("  {key:<4} {:>6.1} min", secs / 60.0);
    }
    println!("rows      : {} single, {} mixed (would be split), {} unlabelled", single, mixed, unlabeled);

    if let Some(path) = args.preview {
        let mut md = String::new();
        writeln!(md, "# {}\n", args.folder.file_name().unwrap_or_default().to_string_lossy())?;
        for (row, label) in rows.iter().zip(&labels) {
            let who = match label {
                RowLabel::Single(k) => k.clone(),
                RowLabel::Mixed { majority, pieces } => {
                    let keys: Vec<&str> = pieces.iter().map(|p| p.key.as_str()).collect();
                    format!("{majority} (split: {})", keys.join(" → "))
                }
                RowLabel::Unlabeled => "?".to_string(),
            };
            writeln!(md, "`{}` **{}**: {}\n", mmss(row.audio_start_time.unwrap_or(0.0)), who, row.text.trim())?;
        }
        std::fs::write(&path, md)?;
        println!("preview   : {}", path.display());
    }
    if matches!(time_map, TimeMap::Unknown) {
        println!("warning   : unknown audio layout, rows labelled on raw times");
    }
    Ok(())
}
