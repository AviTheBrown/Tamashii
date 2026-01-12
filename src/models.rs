use crate::errors::{DatabaseError, FileError};
use crate::pub_struct;
use chrono::{DateTime, Local};
use compio::fs::File;
use exn::Exn;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::hash::Hash;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileRecord {
    pub id: String,
    path: std::path::PathBuf,
    hash: HexStirng,
    size: u8,
    time_stamp: std::time::SystemTime,
}

impl FileRecord {
    fn gen_id() -> String {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = [0; 16];
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
    pub fn new(
        path: std::path::PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: std::time::SystemTime,
    ) -> Self {
        return Self {
            id: Self::gen_id(),
            path,
            hash,
            size,
            time_stamp,
        };
    }
    // TODO! find a away to add file to DB files Vec<FileRecord>
    // only if the file path has no need added to
    // file vec previously
    pub fn add_to_db(&self, db: &Database) -> Result<(), Exn<FileError>> {
        if !db.files.contains(self.path) {
            db.files.push(self.clone());
        }
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
struct FileRecordBuilder<'db> {
    database: &'db mut Database,
    id: Option<String>,
    path: Option<PathBuf>,
    hash: Option<HexStirng>,
    size: Option<u8>,
    time_stamp: Option<std::time::SystemTime>,
}
impl<'db> FileRecordBuilder<'db> {
    fn with_fields(
        mut self,
        id: String,
        path: PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: std::time::SystemTime,
    ) -> Self {
        self.id = Some(id);
        self.path = Some(path);
        self.hash = Some(hash);
        self.size = Some(size);
        self.time_stamp = Some(time_stamp);
        self
    }
    fn validate(&self) -> Result<(), Exn<DatabaseError>> {
        if let Some(v) = &self.id {
            if v.is_empty() {
                return Err(Exn::new(DatabaseError {
                    message: format!("The ID for the file is invalid"),
                }));
            }
            println!("Id is valid");
        }
        if let Some(v) = &self.path {
            match v.metadata() {
                Ok(_) => println!("Path is valid"),
                Err(err) => {
                    return Err(Exn::new(DatabaseError {
                        message: format!("Unable to locate file: {}", err),
                    }));
                }
            }
        }
        if let Some(v) = &self.hash {
            if v.to_string().is_empty() {
                return Err(Exn::new(DatabaseError {
                    message: format!("The HexString for the file is invalid"),
                }));
            }
            println!("The Hex is valid")
        }
        if let Some(v) = self.size {
            if v == 0 {
                println!("This file seems to be empty");
                return Err(Exn::new(DatabaseError {
                    message: format!("The file is 0 bytes try another file"),
                }));
            }
        }
        if let Some(v) = self.time_stamp {
            match v.duration_since(UNIX_EPOCH) {
                Ok(_) => println!("SystemTime is valid"),
                Err(err) => {
                    return Err(Exn::new(DatabaseError {
                        message: format!(
                            "Invalid SystemTime it is earlier than UNIX_EPOCH: {}",
                            err
                        ),
                    }));
                }
            };
        }
        Ok(())
    }
    fn commit(self) -> Result<&'db FileRecord, Exn<DatabaseError>> {}
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub version: String,
    pub root_dir: PathBuf,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub files: Vec<FileRecord>,
}

impl Database {
    fn builder(&mut self) -> FileRecordBuilder;

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
