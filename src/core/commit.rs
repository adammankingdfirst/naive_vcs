use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub tree: String,
    pub parents: Vec<String>,
    pub author: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl Commit {
    pub fn new(
        tree: String,
        parents: Vec<String>,
        author: String,
        message: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tree,
            parents,
            author,
            message,
            timestamp: Utc::now(),
        }
    }

    pub fn is_merge(&self) -> bool {
        self.parents.len() > 1
    }

    pub fn short_hash(&self) -> String {
        self.id.chars().take(8).collect()
    }
}