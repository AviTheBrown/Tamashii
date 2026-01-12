use std::fmt;
use std::path::PathBuf;

/// A trait for types that can be used as paths in `IoError`.
///
/// This trait ensures that any type used for error context is thread-safe,
/// debuggable, and sync.
pub trait AllowedIO: Send + Sync + std::fmt::Debug {}
impl AllowedIO for PathBuf {}
impl AllowedIO for &PathBuf {}

/// An error representing a filesystem I/O failure with path context.
///
/// # Type Parameters
///
/// * `T` - The type of the path, must implement `AllowedIO`
pub struct IoError<T: AllowedIO> {
    /// The path where the I/O error occurred
    pub path: T,
    /// A descriptive error message
    pub message: String,
}

impl<T: AllowedIO + std::fmt::Debug> std::fmt::Display for IoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error on {:?}: {}", self.path, self.message)
    }
}

impl<T: AllowedIO + std::fmt::Debug> std::fmt::Debug for IoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IoError")
            .field("path", &self.path)
            .field("message", &self.message)
            .finish()
    }
}

impl<T: AllowedIO + std::fmt::Debug> std::error::Error for IoError<T> {}

/// Error encountered during application initialization or setup.
#[derive(Debug)]
pub struct InitError {
    /// Details about the initialization failure
    pub message: String,
}

impl std::error::Error for InitError {}
impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Initalization Error: {}", self.message)
    }
}

/// Error encountered during file hashing operations.
#[derive(Debug)]
pub struct HashError {
    /// The specific hashing error message
    pub message: HashErrorMessage,
}

impl std::error::Error for HashError {}
impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hashing Error: {:?}", self.message)
    }
}

/// Specific failure reasons for hashing operations.
#[derive(Debug)]
pub enum HashErrorMessage {
    /// The computation of the hash failed
    ComputationFailed(String),
    /// The input or output format was invalid
    InvalidFormat(String),
}

/// Error related to database operations, serialization, or deserialization.
#[derive(Debug)]
pub struct DatabaseError {
    /// Contextual error message
    pub message: String,
}

impl std::error::Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database Error: {}", self.message)
    }
}

/// Error related to general file handling or validation.
#[derive(Debug)]
pub struct FileError {
    /// Descriptive error message
    pub message: String,
}

impl std::error::Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File Error: {}", self.message)
    }
}

/// Enumeration of errors that can occur during file verification.
pub enum VerificationError {
    /// The file's current hash does not match the stored hash
    HashMissMatched { 
        /// Path to the inconsistent file
        path: PathBuf 
    },
    /// The file exists but is not tracked in the database
    FileUntracked(PathBuf),
    /// An I/O failure occurred during verification
    IoFailure,
    /// A hashing failure occurred during verification
    HashFailure,
}
