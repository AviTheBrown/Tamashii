use crate::errors::IoError;
use crate::files;
use crate::models::HexStirng;
use compio::{fs::File, io::AsyncReadAtExt};
use exn::{Exn, ResultExt};
use std::path::PathBuf;

/// Computes the SHA-256 hash of a file's content asynchronously.
///
/// This function reads the entire file content into memory and computes
/// its SHA-256 digest. It returns the hash as a `HexStirng`.
///
/// # Arguments
///
/// * `file` - A reference to the opened `File` handle
/// * `file_path` - The path to the file (used for error context)
///
/// # Returns
///
/// * `Ok(HexStirng)` - The hex-encoded SHA-256 hash of the file
/// * `Err(Exn<IoError<PathBuf>>)` - If metadata retrieval or file reading fails
///
/// # Errors
///
/// This function will return an error if:
/// * Metadata cannot be retrieved for the file
/// * The file cannot be read from the beginning to the end
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use tamashii::files;
/// use tamashii::hash::hash_file;
///
/// #[compio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let path = Path::new("test.txt");
///     let file = files::get_file(path).await?;
///     let hash = hash_file(&file, path).await?;
///     println!("SHA-256: {}", hash);
///     Ok(())
/// }
/// ```
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
pub fn hash_bytes(bytes: &[u8]) -> HexStirng {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    HexStirng(format!("{:x}", hasher.finalize()))
}
pub async fn hash_file(file: &File) -> Result<HexStirng, Exn<IoError<PathBuf>>> {
    let bytes = read_file_bytes(file).await?;
    Ok(hash_bytes(&bytes))
}
