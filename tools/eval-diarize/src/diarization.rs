//! The app's diarization core, compiled from `frontend/src-tauri/src/diarization`.
#[path = "../../../frontend/src-tauri/src/diarization/assign.rs"]
pub mod assign;
#[path = "../../../frontend/src-tauri/src/diarization/cluster.rs"]
pub mod cluster;
#[path = "../../../frontend/src-tauri/src/diarization/diarizer.rs"]
pub mod diarizer;
#[path = "../../../frontend/src-tauri/src/diarization/embedding.rs"]
pub mod embedding;
#[path = "../../../frontend/src-tauri/src/diarization/fbank.rs"]
pub mod fbank;
#[path = "../../../frontend/src-tauri/src/diarization/reconstruct.rs"]
pub mod reconstruct;
#[path = "../../../frontend/src-tauri/src/diarization/segmentation.rs"]
pub mod segmentation;
#[path = "../../../frontend/src-tauri/src/diarization/timing.rs"]
pub mod timing;

/// The model pins of the app's `diarization/models.rs` that the core reads (names and hashes).
pub mod models {
    pub struct ModelFile {
        pub file_name: &'static str,
        pub sha256: &'static str,
    }
    pub const SEGMENTATION: ModelFile = ModelFile {
        file_name: "segmentation-3.0.onnx",
        sha256: "220ad67ca923bef2fa91f2390c786097bf305bceb5e261d4af67b38e938e1079",
    };
    pub const EMBEDDING: ModelFile = ModelFile {
        file_name: "campplus-voxceleb.onnx",
        sha256: "357a834f702b80161e5b981182c038e18553c1f2ca752ed6cec2052365d4129b",
    };
}

include!(concat!(env!("OUT_DIR"), "/mod_core.rs"));
