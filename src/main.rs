mod hash;
mod models;

#[compio::main]
async fn main() {
    let (hash, value) = hash::hash_file().await.unwrap();
}
