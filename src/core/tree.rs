use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub name: String,
    pub hash: String,
    pub is_file: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub entries: HashMap<String, TreeEntry>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, name: String, hash: String, is_file: bool) {
        let entry = TreeEntry { name: name.clone(), hash, is_file };
        self.entries.insert(name, entry);
    }

    pub fn get_files(&self) -> Vec<&TreeEntry> {
        self.entries.values().filter(|entry| entry.is_file).collect()
    }

    pub fn get_directories(&self) -> Vec<&TreeEntry> {
        self.entries.values().filter(|entry| !entry.is_file).collect()
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}