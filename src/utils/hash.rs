use std::path::Path;
use std::fs;
use sha2::{Sha256, Digest};
use anyhow::Result;

pub fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let data = fs::read(path)?;
    Ok(calculate_hash(&data))
}