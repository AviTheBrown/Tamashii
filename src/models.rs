struct FIleRecord {
    path: std::path::PathBuf,
    hash: String,
    size: u8,
    time_stamp: std::time::SystemTime,
}
