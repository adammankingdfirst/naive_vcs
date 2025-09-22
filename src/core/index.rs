use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::utils::hash::calculate_file_hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub modified: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub entries: HashMap<PathBuf, IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Index> {
        if !path.as_ref().exists() {
            return Ok(Index::new());
        }

        let data = fs::read(path)?;
        if data.is_empty() {
            return Ok(Index::new());
        }

        let json = String::from_utf8(data)?;
        let index: Index = serde_json::from_str(&json)?;
        Ok(index)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, file_path: P, repo_root: P) -> Result<()> {
        let file_path = file_path.as_ref();
        let repo_root = repo_root.as_ref();
        
        if !file_path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {}", file_path.display()));
        }

        let relative_path = file_path.strip_prefix(repo_root)?;
        let metadata = fs::metadata(file_path)?;
        let hash = calculate_file_hash(file_path)?;

        let entry = IndexEntry {
            path: relative_path.to_path_buf(),
            hash,
            size: metadata.len(),
            modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64,
        };

        self.entries.insert(relative_path.to_path_buf(), entry);
        Ok(())
    }

    pub fn remove_file<P: AsRef<Path>>(&mut self, file_path: P) {
        self.entries.remove(file_path.as_ref());
    }

    pub fn is_staged<P: AsRef<Path>>(&self, file_path: P) -> bool {
        self.entries.contains_key(file_path.as_ref())
    }

    pub fn get_staged_files(&self) -> Vec<&PathBuf> {
        self.entries.keys().collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}