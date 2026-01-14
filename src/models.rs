use crate::database::{parse_database_file, serialize_database};
use crate::errors::{DatabaseError, InitError};
use chrono::{DateTime, Local};
use exn::Exn;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
const VERSION: &str = "1.0.0";

/// A wrapper around `String` representing a hex-encoded hash value.
///
/// Provides custom `Hash`, `PartialEq`, and `Display` implementations
/// tailored for hex strings.
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

/// A permanent record of a file stored in the database.
///
/// Includes a unique identifier, path, hash, size, and creation timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileRecord {
    /// Unique 128-bit identifier (hex-encoded)
    pub id: String,
    /// Absolute path to the file
    pub path: std::path::PathBuf,
    /// Content hash of the file
    pub hash: HexStirng,
    /// Size of the file in bytes (up to 255 bytes for this specific implementation)
    pub size: u8,
    /// System time when the file was indexed
    pub time_stamp: std::time::SystemTime,
}

impl FileRecord {
    /// Creates a new `FileRecord` with a generated ID.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    /// * `hash` - Computed hash of the file
    /// * `size` - Size of the file in bytes
    /// * `time_stamp` - Creation/Modification time
    pub fn new(
        path: std::path::PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: std::time::SystemTime,
    ) -> Self {
        return Self {
            id: Database::gen_id(),
            path,
            hash,
            size,
            time_stamp,
        };
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

/// Builder for constructing and validating `FileRecord` instances.
///
/// Ensures that all required fields are present and valid before
/// committing the record to the database.
struct FileRecordBuilder<'db> {
    /// Reference to the target database
    pub database: &'db mut Database,
    /// Optional unique identifier
    pub id: Option<String>,
    /// Optional file path
    pub path: Option<PathBuf>,
    /// Optional file hash
    pub hash: Option<HexStirng>,
    /// Optional file size
    pub size: Option<u8>,
    /// Optional timestamp
    pub time_stamp: Option<std::time::SystemTime>,
}

impl<'db> FileRecordBuilder<'db> {
    /// Populates all builder fields in one call.
    pub fn with_fields(
        mut self,
        path: PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: std::time::SystemTime,
    ) -> Self {
        self.id = Some(Database::gen_id());
        self.path = Some(path);
        self.hash = Some(hash);
        self.size = Some(size);
        self.time_stamp = Some(time_stamp);
        self
    }

    /// Validates the current builder state.
    ///
    /// Checks for empty IDs, existence of path, non-empty hashes,
    /// non-zero sizes, and valid timestamps.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all fields are valid
    /// * `Err(Exn<DatabaseError>)` with a descriptive message if any validation fails
    fn validate(&self) -> Result<(), Exn<DatabaseError>> {
        if let Some(v) = &self.id {
            if v.is_empty() {
                return Err(Exn::new(DatabaseError {
                    message: format!("The ID for the file is invalid"),
                }));
            }
        }
        if let Some(v) = &self.path {
            match v.metadata() {
                Ok(_) => (),
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
        }
        if let Some(v) = self.size {
            if v == 0 {
                return Err(Exn::new(DatabaseError {
                    message: format!("The file is 0 bytes try another file"),
                }));
            }
        }
        if let Some(v) = self.time_stamp {
            match v.duration_since(UNIX_EPOCH) {
                Ok(_) => (),
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

    /// Validates and appends the record to the database.
    ///
    /// # Returns
    ///
    /// * `Ok(&FileRecord)` - A reference to the newly added record
    /// * `Err(Exn<DatabaseError>)` - If validation or insertion fails
    pub fn commit(self) -> Result<&'db FileRecord, Exn<DatabaseError>> {
        match self.validate() {
            Ok(_) => {
                let file_record = FileRecord {
                    id: self.id.unwrap(),
                    path: self.path.unwrap(),
                    hash: self.hash.unwrap(),
                    size: self.size.unwrap(),
                    time_stamp: self.time_stamp.unwrap(),
                };

                self.database.files.push(file_record);
                Ok(self.database.files.last().unwrap())
            }
            Err(err) => Err(Exn::new(DatabaseError {
                message: format!(
                    "There was problem validating and commiting the FileRecord: {}",
                    err
                ),
            })),
        }
    }
}

/// The main database structure storing file tracking information.
///
/// Persisted as a JSON file, typically `.tamashii.json`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    /// Schema version
    pub version: String,
    /// Root directory of the tracked files
    pub root_dir: PathBuf,
    /// Database creation timestamp
    pub created_at: std::time::SystemTime,
    /// Database last update timestamp
    pub updated_at: std::time::SystemTime,
    /// List of tracked file records
    pub files: Vec<FileRecord>,
}

impl Database {
    /// Returns a new `FileRecordBuilder` associated with this database.
    pub fn builder(&mut self) -> FileRecordBuilder {
        FileRecordBuilder {
            database: self,
            id: None,
            path: None,
            hash: None,
            size: None,
            time_stamp: None,
        }
    }

    /// Initializes a new database with current working directory and time.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Initialized database instance
    /// * `Err(Exn<DatabaseError>)` - If the current directory cannot be determined
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
    /// Loads the database from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the `.tamashii.json` database file
    ///
    /// # Returns
    ///
    /// * `Ok(Database)` - The loaded database instance
    /// * `Err(Exn<InitError>)` - If loading or parsing fails
    pub async fn load(path: &PathBuf) -> Result<Database, Exn<InitError>> {
        parse_database_file(path).await.map_err(|db_err| {
            let err_msg = format!("Failed to load DB file: {}", db_err);
            db_err.raise(InitError { message: err_msg })
        })
    }

    /// Saves the current database state to disk.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully saved the database
    /// * `Err(Exn<DatabaseError>)` - If serialization or writing fails
    pub async fn save(&self) -> Result<(), Exn<DatabaseError>> {
        serialize_database(self).await.map_err(|db_err| {
            Exn::new(DatabaseError {
                message: format!("Failed to save the DB: {}", db_err),
            })
        })
    }
    // pub fn add_file()
    /// Generates a random 128-bit hex-encoded ID.
    fn gen_id() -> String {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = [0; 16];
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}
