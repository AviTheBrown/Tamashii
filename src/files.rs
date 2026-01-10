use crate::errors::IoError;
use compio::fs::File;
use compio::fs::Metadata;
use exn::{Exn, ResultExt};
use std::path::{Path, PathBuf};

/// Opens a file asynchronously and wraps any I/O errors with contextual path information.
///
/// This function attempts to open the file at the specified path for reading. If the operation
/// fails (e.g., file doesn't exist, permission denied), it converts the error into an `Exn<IoError<PathBuf>>`
/// with the file path and a descriptive message.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the file to open
///
/// # Returns
///
/// * `Ok(File)` - An opened file handle on success
/// * `Err(Exn<IoError<PathBuf>>)` - An enriched error containing the path and failure message
///
/// # Errors
///
/// This function will return an error if:
/// * The file does not exist at the specified path
/// * The process lacks permission to read the file
/// * The path points to a directory instead of a file
/// * Any other I/O error occurs during the open operation
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
///
/// #[compio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Open an existing file
///     let path = Path::new("tamashii.db");
///     let file = get_file(path).await?;
///     println!("Successfully opened file: {:?}", path);
///     
///     // This will error if the file doesn't exist
///     let missing = Path::new("nonexistent.txt");
///     match get_file(missing).await {
///         Ok(_) => println!("File found"),
///         Err(e) => eprintln!("Error: {}", e), // Prints path info
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn get_file(file_path: &Path) -> Result<File, Exn<IoError<PathBuf>>> {
    File::open(file_path).await.or_raise(|| IoError {
        path: file_path.to_path_buf(),
        message: format!("\nFailed to get file: {:?}", file_path.to_path_buf()),
    })
}
/// Retrieves metadata for an opened file asynchronously with enhanced error context.
///
/// This function queries the filesystem for metadata (size, permissions, modification time, etc.)
/// associated with the provided file handle. Errors are enriched with the original file path
/// to aid debugging.
///
/// # Arguments
///
/// * `file` - A reference to an opened `File` handle
/// * `file_path` - The original path used to open the file (for error reporting)
///
/// # Returns
///
/// * `Ok(Metadata)` - File metadata including size, permissions, and timestamps
/// * `Err(Exn<IoError<PathBuf>>)` - An enriched error with path context if metadata retrieval fails
///
/// # Errors
///
/// This function will return an error if:
/// * The file handle has been closed or is invalid
/// * The underlying filesystem operation fails
/// * Permissions prevent accessing file metadata
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
///
/// #[compio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let path = Path::new("tamashii.db");
///     let file = get_file(path).await?;
///     
///     // Get metadata to check file size
///     let metadata = get_meta(&file, path).await?;
///     println!("File size: {} bytes", metadata.len());
///     println!("Is read-only: {}", metadata.permissions().readonly());
///     println!("Modified: {:?}", metadata.modified()?);
///     
///     Ok(())
/// }
/// ```
///
/// # Visual Flow
///
/// ```text
/// get_file(path)
///     ↓
///   File handle
///     ↓
/// get_meta(&file, path)
///     ↓
///   Metadata { size, permissions, modified_time, ... }
/// ```
pub async fn get_meta(file: &File, file_path: &Path) -> Result<Metadata, Exn<IoError<PathBuf>>> {
    file.metadata().await.or_raise(|| IoError {
        path: file_path.to_path_buf(),
        message: format!("Failed to get metadata from file: {:?}", &file),
    })
}
