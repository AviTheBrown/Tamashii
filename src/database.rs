use crate::errors::DatabaseError;
use crate::models::Database;
use exn::{Exn, ResultExt};
use serde_json;
use std::path::PathBuf;

pub async fn write_database_file(db: &Database) -> Result<(), Exn<DatabaseError>> {
    let json_data = serde_json::to_string_pretty(db).or_raise(|| DatabaseError {
        message: format!("There was an error trying to get the database."),
    })?;
    // creates .tamashii.json if it doesnt exist
    compio::fs::write(PathBuf::from(".tamashii.json"), json_data)
        .await
        .0
        .map_err(|err| {
            let db_error = DatabaseError {
                message: format!("Failed to write to database: {:?}", err),
            };
            Exn::new(db_error)
        })
}

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
