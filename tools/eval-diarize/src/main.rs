//! Diarize one Meetily meeting folder with the PR #833 pipeline and report speed and results.
//! Usage: eval-diarize <meeting_folder> <models_dir> [num_speakers]
pub fn ensure_onnx_runtime_available() -> anyhow::Result<()> {
    Ok(())
}

pub mod api {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct TranscriptSegment {
        pub id: String,
        pub text: String,
        #[serde(default)]
        pub timestamp: String,
        pub audio_start_time: Option<f64>,
        pub audio_end_time: Option<f64>,
        pub duration: Option<f64>,
        #[serde(default)]
        pub speaker: Option<String>,
    }
}

pub mod audio {
    pub mod vad {
        #[derive(Debug, Clone)]
        pub struct SpeechSegment {
            pub samples: Vec<f32>,
            pub start_timestamp_ms: f64,
            pub end_timestamp_ms: f64,
            pub confidence: f32,
        }
    }
    pub mod incremental_saver {
        pub const AUDIO_LAYOUT_FIELD: &str = "audio_layout";
        pub const AUDIO_LAYOUT_SINGLE_STREAM: &str = "single_stream";
    }
}

pub mod diarization;

mod run;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    run::main()
}
