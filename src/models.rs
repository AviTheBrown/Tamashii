use crate::pub_struct;
use chrono::{DateTime, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::PathBuf;

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

#[derive(Debug, Serialize, Deserialize)]
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
impl std::fmt::Display for HexStirng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub_struct! {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FileRecord {
        path: std::path::PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: std::time::SystemTime,
    }
}

impl std::fmt::Display for FileRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime: DateTime<Local> = self.time_stamp.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S");
        write!(
            f,
            "File name: {}\nHash: {}\nSize: {} bytes\nCreated: {}",
            self.path.display(),
            self.hash,
            self.size,
            formatted_time
        )
    }
}
pub_struct! {
#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    version: String,
    root_dir: PathBuf,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    files: Vec<FileRecord>,
}
}

impl Database {
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            root_dir: PathBuf::from("/tmpl/test_db"),
            created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            files: vec![FileRecord {
                path: PathBuf::from("example.txt"),
                hash: HexStirng("deadbeef".to_string()),
                size: 42,
                time_stamp: std::time::SystemTime::now(),
            }],
        }
    }
}
