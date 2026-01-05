use crate::files;
use crate::models::HexStirng;
use compio::{fs::File, io::AsyncReadAtExt};

pub async fn hash_file(file: &File) -> Result<HexStirng, Box<dyn std::error::Error>> {
    let file_meta = files::get_meta(&file).await?;
    let (_, buffer) = file
        .read_to_end_at(Vec::with_capacity(file_meta.len() as usize), 0)
        .await
        .unwrap();

    let hash_value = {
        use sha2::{Digest, Sha256};
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&buffer);
        format!("{:x}", sha256_hasher.finalize())
    };
    Ok(HexStirng(hash_value))
}
