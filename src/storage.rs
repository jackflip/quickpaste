use std::path::{PathBuf};

pub fn storage() -> PathBuf {
    let storage = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");

    PathBuf::from(storage)
}