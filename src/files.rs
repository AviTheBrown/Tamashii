use crate::errors::IoError;
use compio::fs::File;
use compio::fs::Metadata;
use exn::{Exn, ResultExt};
use std::path::{Path, PathBuf};

pub async fn get_file(file_path: &Path) -> Result<File, Exn<IoError<PathBuf>>> {
    File::open(file_path).await.or_raise(|| IoError {
        path: file_path.to_path_buf(),
        message: format!("\nFailed to get file: {:?}", file_path.to_path_buf()),
    })
}
pub async fn get_meta(file: &File, file_path: &Path) -> Result<Metadata, Exn<IoError<PathBuf>>> {
    file.metadata().await.or_raise(|| IoError {
        path: file_path.to_path_buf(),
        message: format!("Failed to get metadata from file: {:?}", &file),
    })
}
