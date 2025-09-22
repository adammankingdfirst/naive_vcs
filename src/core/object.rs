use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::core::{Commit, Tree, Blob};
use crate::utils::hash::calculate_hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    Commit,
    Tree,
    Blob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Object {
    Commit(Commit),
    Tree(Tree),
    Blob(Blob),
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Commit(_) => ObjectType::Commit,
            Object::Tree(_) => ObjectType::Tree,
            Object::Blob(_) => ObjectType::Blob,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_string(self)?;
        Ok(json.into_bytes())
    }

    pub fn deserialize(data: &[u8]) -> Result<Object> {
        let json = String::from_utf8(data.to_vec())?;
        let object: Object = serde_json::from_str(&json)?;
        Ok(object)
    }

    pub fn hash(&self) -> String {
        let data = self.serialize().unwrap();
        calculate_hash(&data)
    }

    pub fn as_commit(&self) -> Option<&Commit> {
        match self {
            Object::Commit(commit) => Some(commit),
            _ => None,
        }
    }

    pub fn as_tree(&self) -> Option<&Tree> {
        match self {
            Object::Tree(tree) => Some(tree),
            _ => None,
        }
    }

    pub fn as_blob(&self) -> Option<&Blob> {
        match self {
            Object::Blob(blob) => Some(blob),
            _ => None,
        }
    }
}