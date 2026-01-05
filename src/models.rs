use std::hash::Hash;

#[derive(Debug)]
pub struct HashedFileInfo {
    pub file_name: String,
    pub hashed_content: HexStirng,
}
impl HashedFileInfo {
    pub fn new(file_name: String, hashed_content: HexStirng) -> Self {
        return Self {
            file_name,
            hashed_content,
        };
    }
}

#[derive(Debug)]
pub struct HexStirng(pub String);
impl Hash for HexStirng {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl PartialEq for HexStirng {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
struct FileRecord {
    path: std::path::PathBuf,
    hash: String,
    size: u8,
    time_stamp: std::time::SystemTime,
}
