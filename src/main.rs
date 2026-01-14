mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use exn::Exn;
use models::{Database, FileRecord};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

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

    let Ok(test_db) = Database::new() else {
        return Err(Exn::new(InitError {
            message: format!("Database::new() failed with error "),
        }));
    };
    test_db.builder().with_fields(
        file_path.to_path_buf(),
        hashed_file_content,
        meta.len() as u8,
        meta.created().expect("Failed to get creation time"),
    );

    match database::serialize_database(&test_db).await {
        Ok(_) => {
            println!("Successfuly wrote to database")
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
