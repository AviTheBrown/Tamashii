use crate::errors::DatabaseError;
use crate::models::Database;
use exn::{Exn, ResultExt};
use serde_json;
use std::path::PathBuf;

pub const DB_PATH: &str = ".tamashii.json";

/// Serializes and writes the database to disk as pretty-printed JSON.
///
/// This function takes a reference to a `Database` instance, serializes it to
/// pretty-printed JSON format, and writes it to the file specified by `DB_PATH`.
/// If the file doesn't exist, it will be created. If it does exist, it will be
/// overwritten.
///
/// # Arguments
///
/// * `db` - A reference to the `Database` to be written to disk
///
/// # Returns
///
/// * `Ok(())` - Database was successfully written to disk
/// * `Err(Exn<DatabaseError>)` - An error occurred during either:
///   - JSON serialization of the database
///   - File write operation
///
/// # Errors
///
/// This function will return an error if:
/// - The database cannot be serialized to JSON (e.g., contains non-serializable data)
/// - The file cannot be written (e.g., insufficient permissions, disk full)
///
/// # Examples
///
/// ```rust
/// use tamashii::models::Database;
/// use tamashii::database::write_database_file;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut db = Database::new();
/// // ... populate database with file entries ...
///
/// write_database_file(&db).await?;
/// println!("Database saved to .tamashii.json");
/// # Ok(())
/// # }
/// ```
pub async fn write_database_file(db: &Database) -> Result<(), Exn<DatabaseError>> {
    let json_data = serde_json::to_string_pretty(db).or_raise(|| DatabaseError {
        message: format!("There was an error trying to get the database."),
    })?;
    // creates .tamashii.json if it doesnt exist
    compio::fs::write(PathBuf::from(DB_PATH), json_data)
        .await
        .0
        .map_err(|err| {
            let db_error = DatabaseError {
                message: format!("Failed to write to database: {:?}", err),
            };
            Exn::new(db_error)
        })
}

/// Reads and deserializes a JSON database file from disk.
///
/// This function reads a JSON file from the specified path, validates that it's
/// valid UTF-8, and deserializes it into a `Database` instance. The function
/// performs validation at each step to ensure data integrity.
///
/// # Arguments
///
/// * `json_file` - A reference to the `PathBuf` pointing to the JSON database file
///
/// # Returns
///
/// * `Ok(Database)` - Successfully parsed database instance
/// * `Err(Exn<DatabaseError>)` - An error occurred during:
///   - File reading
///   - UTF-8 validation
///   - JSON deserialization
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be read (e.g., doesn't exist, insufficient permissions)
/// - The file content is not valid UTF-8
/// - The JSON is malformed or doesn't match the `Database` schema
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use tamashii::database::parse_database_file;
///
/// #[compio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db_path = PathBuf::from(".tamashii.json");
///
///     match parse_database_file(&db_path).await {
///         Ok(database) => {
///             println!("Loaded database version: {}", database.version);
///         }
///         Err(e) => {
///             eprintln!("Failed to load database: {}", e);
///         }
///     }
///     Ok(())
/// }
/// ```
///
/// # Visual Flow
///
/// ```text
/// .tamashii.json (PathBuf)
///     ↓
/// Read raw bytes (compio::fs::read)
///     ↓
/// Validate UTF-8 (str::from_utf8)
///     ↓
/// Deserialize JSON (serde_json::from_str)
///     ↓
/// Database struct
/// ```
pub async fn parse_database_file(json_file: &PathBuf) -> Result<Database, Exn<DatabaseError>> {
    // in byte form
    let json_bytes = compio::fs::read(&json_file)
        .await
        .or_raise(|| DatabaseError {
            message: format!("Unable to parse the json(db) file"),
        })?;

    let json_str = str::from_utf8(&json_bytes).map_err(|err| {
        Exn::new(DatabaseError {
            message: format!("There was an error converting bytes to &str: {}", err),
        })
    })?;
    let database: Database = serde_json::from_str(json_str).map_err(|err| {
        Exn::new(DatabaseError {
            message: format!("Invalid JSON format: {}", err),
        })
    })?;

    Ok(database)
}
