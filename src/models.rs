use std::hash::Hash;

pub struct FilePath(String);
impl Hash for FilePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct HashString(pub String);
impl Hash for HashString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl PartialEq for HashString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
struct FIleRecord {
    path: std::path::PathBuf,
    hash: String,
    size: u8,
    time_stamp: std::time::SystemTime,
}
