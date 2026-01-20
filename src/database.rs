use crate::errors::DatabaseError;
use crate::models::Database;
use exn::{Exn, ResultExt};
use serde_json;
use std::path::{Path, PathBuf};

/// Default filename for the Tamashii database file.
///
/// This JSON file stores the `Database` structure, including versioning,
/// root directory information, and all tracked `FileRecord` entries.
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
pub async fn serialize_database(db: &Database) -> Result<(), Exn<DatabaseError>> {
    let json_data = serde_json::to_string_pretty(db).or_raise(|| DatabaseError {
        message: format!("Failed to serialize database to JSON"),
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
/// valid UTF-8, and deserializes it into a `Database` instance.
///
/// # Arguments
///
/// * `json_file` - A reference to the `Path` pointing to the JSON database file
///
/// # Returns
///
/// * `Ok(Database)` - Successfully parsed database instance
/// * `Err(Exn<DatabaseError>)` - An error occurred during reading or parsing
pub async fn parse_database_file(json_file: &Path) -> Result<Database, Exn<DatabaseError>> {
    let json_bytes = compio::fs::read(&json_file)
        .await
        .or_raise(|| DatabaseError {
            message: format!("Unable to parse the json(db) file"),
        })?;

    let json_str = std::str::from_utf8(&json_bytes).map_err(|err| {
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
