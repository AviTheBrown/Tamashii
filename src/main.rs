mod commands;
use crate::commands::{Cli, Commands};
use ::colored::ColoredString;
mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use clap::{Command, Parser};
use colored::Colorize;
use exn::{Exn, ResultExt};
use models::Database;
use std::path::{Path, PathBuf};

use crate::{database::DB_PATH, errors::InitError};

/// The entry point of the Tamashii CLI application.
///
/// This function parses command-line arguments, calculates the hash of a target file,
/// and updates the local database with a new `FileRecord`.
///
/// # Returns
///
/// * `Ok(())` - Successfully processed the file and updated the database
/// * `Err(Exn<InitError>)` - If any fatal error occurs during execution
#[compio::main]
pub async fn main() {
    // collect args from users
    // for now just one
    // let args: Vec<String> = std::env::args().collect();

    // // path that will be worked on  from args
    // let file_path = Path::new(&args[1]);
    if let Err(e) = run().await {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
pub async fn run() -> Result<(), Exn<InitError>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            println!("Running init...")
        }
        Commands::Add { path } => {
            let green_add = format!("Adding path {}", path.display()).bold().green();
            println!("{}", green_add);
            // get file
            let file = files::get_file(&path).await.or_raise(|| InitError {
                message: format!(
                    "{}\n\t{}",
                    format!("Cannot add {} - file does not exist", path.display())
                        .bold()
                        .red(),
                    format!("Usage: tamashii add <path-to-exisiting-file>")
                        .bold()
                        .yellow()
                ),
            })?;
            // retrieve metadata of file
            let meta = files::get_meta(&file).await.map_err(|err| InitError {
                message: format!("Failed to retrieve metadata: {}", err),
            })?;
            // hash the contents of the file
            let hashed_file_content = hash::hash_file(&file).await.map_err(|err| {
                Exn::new(InitError {
                    message: format!("Failed to hash {:?}'s contents: {}", path, err),
                })
            })?;
            let mut test_db = Database::get_or_create_db(DB_PATH).await?;
            test_db
                .builder()
                .with_fields(
                    path,
                    hashed_file_content,
                    meta.len() as u8,
                    // TODO handle error, get rid of the expect
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
            println!("File added!")
        }
        Commands::Verify { path, all } => match (path, all) {
            (Some(p), false) => {
                // load db
                let db = Database::load(&PathBuf::from(DB_PATH))
                    .await
                    .or_raise(|| InitError {
                        message: format!(" Database failed to load"),
                    })?;
                // open file
                let file = files::get_file(&p).await.or_raise(|| InitError {
                    message: "There was a problem retrieveing the file.".into(),
                })?;
                // hash file
                let current_hash = hash::hash_file(&file).await.or_raise(|| InitError {
                    message: "There was an error hashing the file".into(),
                })?;
                // find file in db if there
                let stored_recored = db.files.iter().find(|file| file.path == p);
                match stored_recored {
                    Some(record) => {
                        if current_hash == record.hash {
                            println!("Hashes match! The file has not changed.")
                        } else {
                            let warning = format!("--- WARNING ---").bold();
                            let warning_msg =
                                format!("Hash mismatch the files have changed.").red();
                            println!("{}", warning);
                            println!("{}", warning_msg);
                            println!(
                                "From ({}...) -> To ({}..)\n Updated on:\n\t {}",
                                &current_hash.0[0..8],
                                &record.hash.0[0..8],
                                record.time_stamp,
                            );
                        }
                    }
                    None => {
                        println!("There was no matching file in the database.")
                    }
                }
            }
            (None, true) => {
                // Verify all - we'll do this after single file works
                println!("Verify all - not implemented yet");
            }
            (None, false) => {
                eprintln!("Error: must provide either <path> or --all");
                std::process::exit(1);
            }
            (Some(_), true) => {
                eprintln!("Error: cannot use both <path> and --all");
                std::process::exit(1);
            }
        },
        Commands::Status => {
            println!("Getting the status...");
            let db = Database::load(&PathBuf::from(&DB_PATH)).await?;
            db.db_status().await;
        }
    }
    Ok(())
}
#[cfg(test)]
mod test {
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
}
