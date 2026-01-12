use crate::errors::IoError;
use crate::files;
use crate::models::HexStirng;
use compio::{fs::File, io::AsyncReadAtExt};
use exn::{Exn, ResultExt};
use std::path::{Path, PathBuf};

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
