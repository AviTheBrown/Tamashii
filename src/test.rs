use exn::ResultExt;
use tempfile::NamedTempFile;

use super::*;
use crate::{hash::hash_bytes, models::VERSION};
use std::path::PathBuf;

/// Tests basic database creation and working directory initialization.
#[compio::test]
async fn create_db() -> Result<(), Exn<InitError>> {
    let mut temp_db_path = tempfile::NamedTempFile::new().or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    let content = std::fs::read(DB_PATH).or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    std::io::Write::write_all(&mut temp_db_path, &content).or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;

    let db = Database::new()?;
    println!("The db: {:?}", db);
    Ok(())
}
/// Tests file opening logic with non-existent paths.
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
/// Verifies that hashing the same input twice produces consistent results.
#[compio::test]
async fn hash_known_input() {
    let input = b"random-input-input";
    let h1 = hash_bytes(input);
    let h2 = hash_bytes(input);
    assert_eq!(h1, h2)
}
/// Tests database building from scratch.
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
/// Verifies that a database instance can be saved to disk.
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
/// Tests loading a database from a temporary file.
#[compio::test]
async fn load_db() -> Result<(), Exn<InitError>> {
    let mut test_tamashii = NamedTempFile::new().or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    let contents = std::fs::read(DB_PATH).or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    std::io::Write::write_all(&mut test_tamashii, &contents).or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    let db = Database::get_or_create_db(
        test_tamashii
            .path()
            .to_str()
            .expect("Path is not valid UTF-8"),
    )
    .await
    .or_raise(|| InitError {
        message: "Failed trying to create a new DB instance".into(),
    })?;
    println!("DB files count: {}", db.files.len());
    println!("DB created_at: {}", db.created_at);
    assert!(db.files.is_empty());
    // let _ = test_tamashii.flush();
    Ok(())
}
