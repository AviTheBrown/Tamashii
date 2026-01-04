use crate::models::HashString;
use std::hash::Hash;
use std::path::Path;

pub async fn hash_file(
    file_path: &Path,
) -> Result<(HashString, String), Box<dyn std::error::Error>> {
    // create a hash that represent the file

    let hash_string = HashString(file_path.to_string_lossy().to_string());
    let hash_value = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        hash_string.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    };
    Ok((hash_string, hash_value))
}
