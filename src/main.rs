mod commands;
use crate::commands::{Cli, Commands};
mod database;
mod errors;
mod files;
mod hash;
mod macros;
mod models;
use clap::Parser;
use colored::Colorize;
use exn::{Exn, ResultExt};
use models::Database;
use std::path::PathBuf;

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
/// Main application logic that handles subcommand routing and execution.
///
/// This function is separated from `main` to facilitate error handling with `Result`.
pub async fn run() -> Result<(), Exn<InitError>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            let db = Database::get_or_create_db(DB_PATH)
                .await
                .or_raise(|| InitError {
                    message: "There was an error trying to create or load the database".into(),
                })?;
            // welcome message
            println!("{}", "✨ Tamashii initialized! ✨ ".green().bold());
            println!("\n{}", "File integrity checker ready.".bright_cyan().bold());

            // usage examples
            println!("\n{}", "Getting started:".cyan().bold());
            println!("  tamashii add <file>     - Track a file");
            println!("  tamashii status         - View tracked files");
            println!("  tamashii verify <file>  - Check if file changed");
            println!("  tamashii verify --all   - Check all tracked files");

            // database info
            println!("\n{} {}", "Database:".cyan().bold(), DB_PATH);
            println!("{} file(s) currently tracked", db.files.len());
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
                let db = Database::load(&PathBuf::from(DB_PATH))
                    .await
                    .or_raise(|| InitError {
                        message: format!(" Database failed to load"),
                    })?;
                let file_len = format!("==== Total of {} files tracked. ====", db.files.len())
                    .bold()
                    .bright_green();
                println!("{}", file_len);
                // iter throuh files
                for file in db.files.iter() {
                    // open each file
                    let f = files::get_file(&file.path).await.or_raise(|| InitError {
                        message: format!(
                            "There was a problem trying to retrieve: {}",
                            file.path.display()
                        )
                        .into(),
                    })?;
                    // hash file
                    let current_hash = hash::hash_file(&f).await.or_raise(|| InitError {
                        message: "There was an error hashing the file".into(),
                    })?;
                    if current_hash == file.hash {
                        let good = format!("--- GOOD ---").bold();
                        let good_msg = format!("Hashes match,").green();
                        let no_change = format!("the files have not changed");
                        println!("{}", good);
                        println!("{} {}", good_msg, no_change);
                        println!("File: {}", file.path.display());
                        println!("Tracked on:\n\t {}", file.time_stamp);
                    } else {
                        let warning = format!("--- WARNING ---").bold();
                        let warning_msg = format!("Hash mismatch the files have changed.").red();
                        println!("{}", warning);
                        println!("{}", warning_msg);
                        println!("File: {}", file.path.display());
                        println!(
                            "From ({}...) -> To ({}..)\n Updated on:\n\t {}",
                            &current_hash.0[0..8],
                            &file.hash.0[0..8],
                            file.time_stamp,
                        );
                    }
                    //TODO add summary
                }
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
mod test;
