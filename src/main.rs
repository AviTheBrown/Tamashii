mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use chrono::{DateTime, TimeZone, Utc};
use models::{Database, FileRecord};
use std::path::{Path, PathBuf};

use crate::models::HexStirng;

#[compio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = Path::new(&args[1]);

    let file = files::get_file(file_path)
        .await
        .expect("Failed to open file");
    let meta = files::get_meta(&file, &file_path)
        .await
        .expect("Failed to retreive file metadata.");
    let hashed_content = hash::hash_file(&file, &file_path).await.unwrap();

    let file_hash_stuff = models::HashedFileInfo::new(args[1].clone(), hashed_content);
    let record = FileRecord {
        path: file_path.to_path_buf(),
        hash: file_hash_stuff.hashed_content,
        size: (meta.len() as u8),
        time_stamp: meta.created().expect("Failed to get creation time"),
    };
    println!("{}", record);
    let test_db = Database {
        version: "1.0.0-test".into(),
        root_dir: PathBuf::from("/tmp/test_db"),
        created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        files: vec![FileRecord {
            path: PathBuf::from("example.txt"),
            hash: HexStirng("deadbeef".to_string()),
            size: 42,
            time_stamp: std::time::SystemTime::now(),
        }],
    };
    match database::write_database_file(&test_db).await {
        Ok(db) => {
            println!("sucess")
        }
        Err(err) => eprintln!("failed"),
    }
    database::parse_database_file(json_file);
}
