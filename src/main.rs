mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use exn::Exn;
use models::{Database, FileRecord};
use std::path::{Path, PathBuf};

use crate::{database::DB_PATH, errors::InitError};

#[compio::main]
async fn main() -> Result<(), Exn<InitError>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = Path::new(&args[1]);

    //TODO turn into exn error
    let file = files::get_file(file_path)
        .await
        .expect("Failed to open file");
    //TODO turn into exn error
    let meta = files::get_meta(&file, &file_path)
        .await
        .expect("Failed to retreive file metadata.");
    //TODO turn into exn error
    let hashed_content = hash::hash_file(&file, &file_path).await.unwrap();

    let file_hash_stuff = models::HashedFileInfo::new(args[1].clone(), hashed_content);
    let record = FileRecord::new(
        file_path.to_path_buf(),
        file_hash_stuff.hashed_content,
        meta.len() as u8,
        meta.created().expect("Failed to get creation time"),
    );

    let Ok(test_db) = Database::new() else {
        return Err(Exn::new(InitError {
            message: format!("Database::new() failed with error "),
        }));
    };
    match database::write_database_file(&test_db).await {
        Ok(_) => {
            println!("Successfuly wrote to databas")
        }
        Err(_) => eprintln!("Failed to write to database"),
    }
    match database::parse_database_file(&PathBuf::from(DB_PATH)).await {
        Ok(parsed_db) => {
            test_db.add_file(&file);
            println!("   The Database was successfully parsed.");
            println!("   Version: {}", parsed_db.version);
            println!("   Root dir: {:?}", parsed_db.root_dir);
            println!("   Files tracked: {}", parsed_db.files.len());

            for file in &parsed_db.files {
                println!("{:?} ({})", file.path, file.hash);
            }
            return Ok(());
        }
        Err(err) => {
            // let init_error = Exn::new(InitError {
            //     message: format!("There was an error parsing the database file: {}", err),
            // });
            return Err(Exn::new(InitError {
                message: format!("There was an error parsing the database file: {}", err),
            }));
        }
    };
}
