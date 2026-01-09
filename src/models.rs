use crate::errors::DatabaseError;
use crate::pub_struct;
use chrono::{DateTime, Local, TimeZone, Utc};
use exn::Exn;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::PathBuf;
const VERSION: &str = "1.0.0";

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
    created_at: std::time::SystemTime,
    updated_at: std::time::SystemTime,
    files: Vec<FileRecord>,
}
}

impl Database {
    pub fn new() -> Result<Self, Exn<DatabaseError>> {
        let current_dir = std::env::current_dir().map_err(|err| {
            Exn::new(DatabaseError {
                message: format!("Failed to get current directory: {}", err),
            })
        })?;
        Ok(Self {
            version: VERSION.to_string(),
            root_dir: PathBuf::from(current_dir),
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
            files: vec![],
        })
    }
}
