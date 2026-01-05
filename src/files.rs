use compio::fs::File;
use std::error::Error;
use std::path::Path;

pub async fn get_file(file_path: &Path) -> Result<File, Box<dyn Error>> {
    File::open(file_path).await.map_err(Into::into)
}
pub async fn get_meta(file: &File) -> Result<compio::fs::Metadata, std::io::Error> {
    file.metadata().await.map_err(|err| {
        eprintln!("Failed to get metadata: {}", err);
        err
    })
}
