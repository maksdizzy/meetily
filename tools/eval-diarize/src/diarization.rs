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

pub mod models {
    pub struct ModelFile {
        pub file_name: &'static str,
    }
    pub const SEGMENTATION: ModelFile = ModelFile { file_name: "segmentation-3.0.onnx" };
    pub const EMBEDDING: ModelFile = ModelFile { file_name: "campplus-voxceleb.onnx" };
}

include!(concat!(env!("OUT_DIR"), "/mod_core.rs"));
