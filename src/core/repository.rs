use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use crate::core::{Index, Object, Commit, Tree, Blob, Branch};
use crate::utils::hash::calculate_hash;

pub struct Repository {
    pub root: PathBuf,
    pub nvcs_dir: PathBuf,
    pub objects_dir: PathBuf,
    pub refs_dir: PathBuf,
    pub head_file: PathBuf,
    pub index_file: PathBuf,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let root = path.as_ref().to_path_buf();
        let nvcs_dir = root.join(".nvcs");
        let objects_dir = nvcs_dir.join("objects");
        let refs_dir = nvcs_dir.join("refs");
        let head_file = nvcs_dir.join("HEAD");
        let index_file = nvcs_dir.join("index");

        Self {
            root,
            nvcs_dir,
            objects_dir,
            refs_dir,
            head_file,
            index_file,
        }
    }

    pub fn init(&self) -> Result<()> {
        if self.nvcs_dir.exists() {
            return Err(anyhow::anyhow!("Repository already exists"));
        }

        fs::create_dir_all(&self.objects_dir)
            .context("Failed to create objects directory")?;
        fs::create_dir_all(&self.refs_dir.join("heads"))
            .context("Failed to create refs/heads directory")?;
        fs::create_dir_all(&self.refs_dir.join("tags"))
            .context("Failed to create refs/tags directory")?;

        // Initialize HEAD to point to main branch
        fs::write(&self.head_file, "ref: refs/heads/main\n")
            .context("Failed to create HEAD file")?;

        // Create empty index
        let index = Index::new();
        index.save(&self.index_file)?;

        println!("Initialized empty repository in {}", self.nvcs_dir.display());
        Ok(())
    }

    pub fn find_repository() -> Result<Repository> {
        let mut current = std::env::current_dir()?;
        
        loop {
            let nvcs_dir = current.join(".nvcs");
            if nvcs_dir.exists() && nvcs_dir.is_dir() {
                return Ok(Repository::new(current));
            }
            
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => return Err(anyhow::anyhow!("Not in a repository")),
            }
        }
    }

    pub fn load_index(&self) -> Result<Index> {
        Index::load(&self.index_file)
    }

    pub fn save_index(&self, index: &Index) -> Result<()> {
        index.save(&self.index_file)
    }

    pub fn store_object(&self, object: &Object) -> Result<String> {
        let hash = object.hash();
        let (dir, file) = hash.split_at(2);
        let object_dir = self.objects_dir.join(dir);
        let object_file = object_dir.join(file);

        if !object_file.exists() {
            fs::create_dir_all(&object_dir)?;
            fs::write(&object_file, object.serialize()?)?;
        }

        Ok(hash)
    }

    pub fn load_object(&self, hash: &str) -> Result<Object> {
        let (dir, file) = hash.split_at(2);
        let object_file = self.objects_dir.join(dir).join(file);
        
        if !object_file.exists() {
            return Err(anyhow::anyhow!("Object {} not found", hash));
        }

        let data = fs::read(&object_file)?;
        Object::deserialize(&data)
    }

    pub fn get_head(&self) -> Result<Option<String>> {
        if !self.head_file.exists() {
            return Ok(None);
        }

        let head_content = fs::read_to_string(&self.head_file)?;
        let head_content = head_content.trim();

        if head_content.starts_with("ref: ") {
            let ref_path = head_content.strip_prefix("ref: ").unwrap();
            let ref_file = self.nvcs_dir.join(ref_path);
            
            if ref_file.exists() {
                let commit_hash = fs::read_to_string(&ref_file)?;
                Ok(Some(commit_hash.trim().to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(head_content.to_string()))
        }
    }

    pub fn get_current_branch(&self) -> Result<Option<String>> {
        if !self.head_file.exists() {
            return Ok(None);
        }

        let head_content = fs::read_to_string(&self.head_file)?;
        let head_content = head_content.trim();

        if head_content.starts_with("ref: refs/heads/") {
            let branch_name = head_content.strip_prefix("ref: refs/heads/").unwrap();
            Ok(Some(branch_name.to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn update_ref(&self, ref_name: &str, commit_hash: &str) -> Result<()> {
        let ref_file = self.nvcs_dir.join(ref_name);
        if let Some(parent) = ref_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&ref_file, format!("{}\n", commit_hash))?;
        Ok(())
    }

    pub fn list_branches(&self) -> Result<Vec<String>> {
        let heads_dir = self.refs_dir.join("heads");
        if !heads_dir.exists() {
            return Ok(vec![]);
        }

        let mut branches = Vec::new();
        for entry in fs::read_dir(&heads_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    branches.push(name.to_string());
                }
            }
        }
        branches.sort();
        Ok(branches)
    }

    pub fn create_branch(&self, name: &str, commit_hash: &str) -> Result<()> {
        let branch_file = self.refs_dir.join("heads").join(name);
        if branch_file.exists() {
            return Err(anyhow::anyhow!("Branch '{}' already exists", name));
        }
        fs::write(&branch_file, format!("{}\n", commit_hash))?;
        Ok(())
    }

    pub fn delete_branch(&self, name: &str) -> Result<()> {
        let current_branch = self.get_current_branch()?;
        if current_branch.as_deref() == Some(name) {
            return Err(anyhow::anyhow!("Cannot delete current branch '{}'", name));
        }

        let branch_file = self.refs_dir.join("heads").join(name);
        if !branch_file.exists() {
            return Err(anyhow::anyhow!("Branch '{}' does not exist", name));
        }
        fs::remove_file(&branch_file)?;
        Ok(())
    }

    pub fn checkout_branch(&self, name: &str) -> Result<()> {
        let branch_file = self.refs_dir.join("heads").join(name);
        if !branch_file.exists() {
            return Err(anyhow::anyhow!("Branch '{}' does not exist", name));
        }

        fs::write(&self.head_file, format!("ref: refs/heads/{}\n", name))?;
        Ok(())
    }
}