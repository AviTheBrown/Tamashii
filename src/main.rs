mod files;
mod hash;
mod models;
use std::path::Path;

#[compio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);
    let file_path = Path::new(&args[1]);
    let file = files::get_file(file_path)
        .await
        .expect("Failed to open file");
    let hashed_content = hash::hash_file(&file).await.unwrap();

    // args.remove(1)
    let file_hash_stuff = models::HashedFileInfo::new(args[1].clone(), hashed_content);
    println!(
        "INFO: \nFile Name{:?}\nHashed Content: {:?}",
        file_hash_stuff.file_name, file_hash_stuff.hashed_content.0
    );
}
