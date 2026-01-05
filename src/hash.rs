use crate::errors::IoError;
use crate::files;
use crate::models::HexStirng;
use compio::{fs::File, io::AsyncReadAtExt};
use exn::{Exn, ResultExt};
use std::path::{Path, PathBuf};

pub async fn hash_file(file: &File, file_path: &Path) -> Result<HexStirng, Exn<IoError<PathBuf>>> {
    let file_meta = files::get_meta(&file, &file_path)
        .await
        .or_raise(|| IoError {
            path: file_path.to_path_buf(),
            message: format!("Unable to retrieve meta data from: {:?}", &file_path),
        })?;
    let (_, buffer) = file
        .read_to_end_at(Vec::with_capacity(file_meta.len() as usize), 0)
        .await
        .unwrap();

    let hash_value = {
        use sha2::{Digest, Sha256};
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&buffer);
        format!("{:x}", sha256_hasher.finalize())
    };
    Ok(HexStirng(hash_value))
}
