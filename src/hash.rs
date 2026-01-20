use crate::errors::IoError;
use crate::files;
use crate::models::HexStirng;
use compio::{fs::File, io::AsyncReadAtExt};
use exn::{Exn, ResultExt};
use std::path::PathBuf;

/// Reads all bytes from a file asynchronously.
///
/// # Arguments
///
/// * `file` - A reference to the opened `File` handle
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The binary content of the file
/// * `Err(Exn<IoError<PathBuf>>)` - If metadata retrieval or file reading fails
pub async fn read_file_bytes(file: &File) -> Result<Vec<u8>, Exn<IoError<PathBuf>>> {
    let file_meta = files::get_meta(&file).await.or_raise(|| IoError {
        path: None,
        message: format!("Unable to retrieve meta data from: {:?}", &file),
    })?;
    let (_, buffer) = file
        .read_to_end_at(Vec::with_capacity(file_meta.len() as usize), 0)
        .await
        .expect("Unable to read file");
    Ok(buffer)
}

/// Computes the SHA-256 hash of a byte slice.
///
/// # Arguments
///
/// * `bytes` - The byte slice to hash
///
/// # Returns
///
/// * `HexStirng` - The hex-encoded SHA-256 hash
pub fn hash_bytes(bytes: &[u8]) -> HexStirng {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    HexStirng(format!("{:x}", hasher.finalize()))
}

/// Computes the SHA-256 hash of a file's content asynchronously.
///
/// This function reads the entire file content and computes its SHA-256 digest.
///
/// # Arguments
///
/// * `file` - A reference to the opened `File` handle
///
/// # Returns
///
/// * `Ok(HexStirng)` - The hex-encoded SHA-256 hash
/// * `Err(Exn<IoError<PathBuf>>)` - If reading the file fails
pub async fn hash_file(file: &File) -> Result<HexStirng, Exn<IoError<PathBuf>>> {
    let bytes = read_file_bytes(file).await?;
    Ok(hash_bytes(&bytes))
}
