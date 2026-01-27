use crate::database::{parse_database_file, serialize_database};
use crate::errors::{DatabaseError, InitError};
use chrono::{DateTime, Utc};
use exn::Exn;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::PathBuf;
pub const VERSION: &str = "1.0.0";

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
    /// Time when the file was indexed
    pub time_stamp: DateTime<Utc>,
}

impl FileRecord {}

impl std::fmt::Display for FileRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_time = self.time_stamp.format("%Y-%m-%d %H:%M:%S");
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
pub struct FileRecordBuilder<'db> {
    /// Reference to the target database
    pub db: &'db mut Database,
    /// Optional file path
    pub id: Option<String>,
    pub path: Option<PathBuf>,
    /// Optional file hash
    pub hash: Option<HexStirng>,
    /// Optional file size
    pub size: Option<u8>,
    /// Optional timestamp
    pub time_stamp: Option<DateTime<Utc>>,
}

impl<'db> FileRecordBuilder<'db> {
    /// Populates all builder fields in one call and generates a new unique ID.
    ///
    /// # Arguments
    ///
    /// * `path` - The absolute path of the file to record
    /// * `hash` - The computed hash of the file
    /// * `size` - The size of the file in bytes
    /// * `time_stamp` - The creation or indexing timestamp
    pub fn with_fields(
        mut self,
        path: PathBuf,
        hash: HexStirng,
        size: u8,
        time_stamp: DateTime<Utc>,
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
    /// Ensures that all required fields (ID, path, hash, size, and timestamp)
    /// have been populated before committing to the database.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all fields are present
    /// * `Err(Exn<DatabaseError>)` with a descriptive message if any field is missing
    pub fn validate(&self) -> Result<(), Exn<DatabaseError>> {
        if self.id.is_none() {
            return Err(Exn::new(DatabaseError {
                message: "ID is missing".into(),
            }));
        }
        if self.path.is_none() {
            return Err(Exn::new(DatabaseError {
                message: "Path is missing".into(),
            }));
        }

        if self.hash.is_none() {
            return Err(Exn::new(DatabaseError {
                message: "Hash is missing".into(),
            }));
        }

        if self.size.is_none() {
            return Err(Exn::new(DatabaseError {
                message: "Size is missing".into(),
            }));
        }

        if self.time_stamp.is_none() {
            return Err(Exn::new(DatabaseError {
                message: "Timestamp is missing".into(),
            }));
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
        self.validate()?;

        let record = FileRecord {
            id: self.id.unwrap(),
            path: self.path.unwrap(),
            hash: self.hash.unwrap(),
            size: self.size.unwrap(),
            time_stamp: self.time_stamp.unwrap(),
        };

        self.db.files.push(record);
        self.db.updated_at = chrono::Utc::now();
        Ok(self.db.files.last().unwrap())
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
    pub created_at: DateTime<Utc>,
    /// Database last update timestamp
    pub updated_at: DateTime<Utc>,
    /// List of tracked file records
    pub files: Vec<FileRecord>,
}

impl Database {
    /// Prints the current status of the database and its tracked files.
    ///
    /// If no files are tracked, it suggests the usage command.
    /// Otherwise, it displays total files, creation date, last update,
    /// and a summarized list of tracked files with their hash prefixes.
    pub async fn db_status(&self) {
        use colored::Colorize;
        if self.files.is_empty() {
            let no_file =
                "No files tracked yet. Use 'tamashii add <file>' to start tracking.".red();
            println!("{}", no_file);
        } else {
            let db_stats = "======= Database Status =======".bold().green();
            let status_display = format!(
                "Total files: {}\nCreated: {}\nLast updated: {}",
                self.files.len(),
                self.created_at,
                self.updated_at
            );
            println!("{}", db_stats);
            println!("{}", status_display);

            let files = self.files.iter();
            for file in files {
                let str_hash = file.hash.to_string();
                let part = &str_hash[0..8];
                println!("File: {} Hash: ({}...)", file.path.display(), part)
            }
            let db_stats1 = "======= Database Status =======".bold().green();
            println!("{}", db_stats1);
        }
    }
    /// Returns an existing database from the specified path or creates a new one if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The file system path where the database file is located
    ///
    /// # Returns
    ///
    /// * `Ok(Database)` - The loaded or newly created database instance
    /// * `Err(Exn<InitError>)` - If loading or initialization fails
    pub async fn get_or_create_db(path: &str) -> Result<Database, Exn<InitError>> {
        let path_ = std::path::Path::new(path);
        if !path_.exists() {
            Self::new()
        } else {
            Self::load(&path_.to_path_buf()).await
        }
    }
    /// Returns a new `FileRecordBuilder` associated with this database.
    ///
    /// The builder is used to create and validate `FileRecord` instances before
    /// adding them to the database.
    pub fn builder(&mut self) -> FileRecordBuilder<'_> {
        FileRecordBuilder {
            db: self,
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
    pub fn new() -> Result<Self, Exn<InitError>> {
        let current_dir = std::env::current_dir().map_err(|err| {
            Exn::new(InitError {
                message: format!("Failed to get current directory: {}", err),
            })
        })?;
        Ok(Self {
            version: VERSION.to_string(),
            root_dir: PathBuf::from(current_dir),
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
    pub async fn load(path: &PathBuf) -> Result<Self, Exn<InitError>> {
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
        serialize_database(self).await
    }
    /// Generates a random 128-bit hex-encoded ID used for unique file identification.
    fn gen_id() -> String {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = [0; 16];
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}
