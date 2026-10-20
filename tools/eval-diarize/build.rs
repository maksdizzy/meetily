//! Copies the shared items of the app's `diarization/mod.rs` (Turn, Cancelled, session helpers)
//! into OUT_DIR, leaving out its app-only submodule declarations and its tests.
use std::path::PathBuf;

fn main() {
    let src = PathBuf::from("../../frontend/src-tauri/src/diarization/mod.rs");
    println!("cargo:rerun-if-changed={}", src.display());
    let text = std::fs::read_to_string(&src).expect("read diarization/mod.rs");
    let start = text.find("use serde::").expect("diarization/mod.rs: `use serde::` not found");
    let end = text.find("#[cfg(test)]").unwrap_or(text.len());
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("mod_core.rs");
    std::fs::write(out, &text[start..end]).expect("write mod_core.rs");
}
