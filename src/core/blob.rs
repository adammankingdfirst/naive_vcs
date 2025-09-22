use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }

    pub fn from_string(content: String) -> Self {
        Self::new(content.into_bytes())
    }

    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.content.clone())
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }
}