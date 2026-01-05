use std::path::PathBuf;

pub trait AllowedIO: Send + Sync + std::fmt::Debug {}
impl AllowedIO for PathBuf {}
impl AllowedIO for &PathBuf {}

pub struct IoError<T: AllowedIO> {
    pub path: T,
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

pub enum HashError {
    ComputationFailed,
    InvalidFormat,
}

pub enum VerificationError {
    HashMissMatched { path: PathBuf },
    FileUntracked(PathBuf),
    IoFailure,
    HashFailure,
}
