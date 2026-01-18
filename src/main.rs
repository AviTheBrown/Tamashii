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
    let file = files::get_file(&file_path).await.map_err(|err| InitError {
        message: format!("There was an error trying to use the file: {}", err),
    })?;
    // retrieve metadata of file
    let meta = files::get_meta(&file).await.map_err(|err| InitError {
        message: format!("Failed to retriece metadata: {}", err),
    })?;
    // hash the contents of the file
    let hashed_file_content = hash::hash_file(&file).await.map_err(|err| {
        Exn::new(InitError {
            message: format!("Failed to hash {:?}'s contents: {}", file_path, err),
        })
    })?;

    let mut test_db = Database::get_or_create_db(DB_PATH).await?;
    test_db
        .builder()
        .with_fields(
            file_path.to_path_buf(),
            hashed_file_content,
            meta.len() as u8,
            meta.created().expect("Failed to get creation time").into(),
        )
        .commit()
        .map_err(|err| {
            Exn::new(InitError {
                message: format!("Failed to commit database changes: {}", err),
            })
        })?;

    test_db.save().await.map_err(|err| {
        Exn::new(InitError {
            message: format!("Failed to save database: {}", err),
        })
    })?;
    Ok(())
}
#[cfg(test)]
mod test {
    use chrono::Utc;
    use exn::ResultExt;

    use super::*;
    use crate::{
        hash::hash_bytes,
        models::{HexStirng, VERSION},
    };
    use std::{
        env::{current_dir, temp_dir},
        io::Write,
        path::PathBuf,
    };

    #[compio::test]
    async fn create_db() -> Result<(), Exn<InitError>> {
        let db = Database::get_or_create_db(DB_PATH).await?;
        println!("The db: {:?}", db);
        Ok(())
    }
    #[compio::test]
    async fn create_file() -> Result<(), Exn<errors::IoError<PathBuf>>> {
        let tmp = tempfile::tempdir().map_err(|err| errors::IoError {
            path: None,
            message: format!("{}", err),
        })?;
        let path = tmp.path().join("test.rs");
        let _ = files::get_file(&path).await;
        Ok(())
    }
    #[compio::test]
    async fn hash_known_input() {
        let input = b"random-input-input";
        let h1 = hash_bytes(input);
        let h2 = hash_bytes(input);
        assert_eq!(h1, h2)
    }
    #[compio::test]
    async fn build_db() -> Result<(), Exn<InitError>> {
        let test_db = Database::new().or_raise(|| InitError {
            message: "Failed trying to create a new DB instance".into(),
        })?;
        let current_dir = std::env::current_dir().map_err(|err| {
            Exn::new(InitError {
                message: format!("Failed to get current directory: {}", err),
            })
        })?;
        assert_eq!(test_db.version, VERSION);
        assert_eq!(test_db.root_dir, PathBuf::from(current_dir));
        Ok(())
    }
    #[compio::test]
    async fn save_db() -> Result<(), Exn<InitError>> {
        let test_db = Database::new().or_raise(|| InitError {
            message: "Failed trying to create a new DB instance".into(),
        })?;
        let _ = test_db.save().await.or_raise(|| InitError {
            message: "Failed to save DB".into(),
        });
        Ok(())
    }
}
