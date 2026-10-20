# eval-diarize

Runs the app's speaker diarization (from `frontend/src-tauri/src/diarization`) on one recorded
Meetily meeting folder, outside the Tauri app, and reports speed, speakers and how transcript rows
would be labelled. Useful for checking accuracy on your own meetings and for tuning thresholds.

Needs `ffmpeg`/`ffprobe` on PATH and the two diarization models (the app downloads them to
`<app data>/models/diarization`).

```bash
cd tools/eval-diarize
cargo run --release -- "<meeting folder>" "<models dir>" [num_speakers|auto] [preview.md]
```

The preview file lists every transcript row with its speaker, and marks rows the app would split
at a speaker change.
