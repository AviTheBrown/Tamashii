mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use exn::Exn;
use models::Database;
use std::path::Path;

use crate::{database::DB_PATH, errors::InitError};

#[compio::main]
async fn main() -> Result<(), Exn<InitError>> {
    // collect args from users
    // for now just one
    let args: Vec<String> = std::env::args().collect();

    // path that will be worked on  from args
    let file_path = Path::new(&args[1]);

    // get file
    let file = files::get_file(file_path).await.map_err(|err| InitError {
        message: format!("There was an error trying to use the file: {}", err),
    })?;
    // retrieve metadata of file
    let meta = files::get_meta(&file, &file_path)
        .await
        .map_err(|err| InitError {
            message: format!("Failed to retriece metadata: {}", err),
        })?;
    // hash the contents of the file
    let hashed_file_content = hash::hash_file(&file, &file_path).await.map_err(|err| {
        Exn::new(InitError {
            message: format!("Failed to hash {:?}'s contents: {}", file_path, err),
        })
    })?;

    // create a instace of FileRecord
    // let record = FileRecord::new(
    //     file_path.to_path_buf(),
    //     hashed_file_content,
    //     // file_hash_stuff.hashed_content,
    //     meta.len() as u8,
    //     meta.created().expect("Failed to get creation time"),
    // );

    let mut test_db = Database::get_or_create_db(DB_PATH).await?;
    test_db
        .builder()
        .with_fields(
            file_path.to_path_buf(),
            hashed_file_content,
            meta.len() as u8,
            meta.created().expect("Failed to get creation time").into(),
        )
        .commit().map_err(|err| Exn::new(InitError {
            message: format!("Failed to commit database changes: {}", err),
        }))?;

    test_db.save().await.map_err(|err| Exn::new(InitError {
        message: format!("Failed to save database: {}", err),
    }))?;

    Ok(())
}
