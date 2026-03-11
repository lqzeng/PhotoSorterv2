// for scanning folders

use walkdir::WalkDir;
use std::path::PathBuf;

pub fn scan_images(dir: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| {
                    let ext = ext.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg" || ext == "png"
                })
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}