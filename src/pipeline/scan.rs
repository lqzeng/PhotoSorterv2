use walkdir::WalkDir;
use std::path::PathBuf;

#[allow(dead_code)] // allow non use of fields for now
pub struct ScanResult {
    pub images: Vec<PathBuf>,
    pub folder_count: usize,
    pub image_count: usize,
}

pub fn scan_images(dir: &str) -> ScanResult {
    let mut images = Vec::new();
    let mut folders = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {

        if entry.file_type().is_dir() {
            folders += 1;
            continue;
        }

        if let Some(ext) = entry.path().extension() {
            let ext = ext.to_string_lossy().to_lowercase();

            if ext == "jpg" || ext == "jpeg" || ext == "png" {
                images.push(entry.path().to_path_buf());
            }
        }
    }

    let image_count = images.len();

    ScanResult {
        images,
        folder_count: folders,
        image_count,
    }
}