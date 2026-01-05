mod errors;
mod files;
mod hash;
mod macros;
mod models;
use models::FileRecord;
use std::path::Path;

#[compio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = Path::new(&args[1]);

    let file = files::get_file(file_path)
        .await
        .expect("Failed to open file");
    let meta = files::get_meta(&file)
        .await
        .expect("Failed to retreive file metadata.");
    let hashed_content = hash::hash_file(&file).await.unwrap();

    // args.remove(1)
    let file_hash_stuff = models::HashedFileInfo::new(args[1].clone(), hashed_content);
    let record = FileRecord {
        path: file_path.to_path_buf(),
        hash: file_hash_stuff.hashed_content,
        size: (meta.len() as u8),
        time_stamp: meta.created().expect("Failed to get creation time"),
    };
    println!("{}", record);
}
