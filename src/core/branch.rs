use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: String,
}

impl Branch {
    pub fn new(name: String, commit: String) -> Self {
        Self { name, commit }
    }
}