use std::fs;
use anyhow::Result;
use crate::core::{Repository, Object, Commit, Tree, Blob};

pub fn execute(message: String, author: Option<String>) -> Result<()> {
    let repo = Repository::find_repository()?;
    let index = repo.load_index()?;

    if index.entries.is_empty() {
        return Err(anyhow::anyhow!("No changes staged for commit"));
    }

    // Create tree from index
    let mut tree = Tree::new();
    for (path, entry) in &index.entries {
        let file_path = repo.root.join(path);
        let content = fs::read(&file_path)?;
        let blob = Blob::new(content);
        let blob_obj = Object::Blob(blob);
        let blob_hash = repo.store_object(&blob_obj)?;
        
        tree.add_entry(
            path.to_string_lossy().to_string(),
            blob_hash,
            true,
        );
    }

    let tree_obj = Object::Tree(tree);
    let tree_hash = repo.store_object(&tree_obj)?;

    // Get parent commit
    let parent_commits = match repo.get_head()? {
        Some(head_commit) => vec![head_commit],
        None => vec![],
    };

    // Create commit
    let author_name = author.unwrap_or_else(|| "Unknown".to_string());
    let commit = Commit::new(tree_hash, parent_commits, author_name, message);
    let commit_obj = Object::Commit(commit.clone());
    let commit_hash = repo.store_object(&commit_obj)?;

    // Update HEAD
    if let Some(current_branch) = repo.get_current_branch()? {
        repo.update_ref(&format!("refs/heads/{}", current_branch), &commit_hash)?;
    } else {
        fs::write(&repo.head_file, format!("{}\n", commit_hash))?;
    }

    // Clear index
    let mut empty_index = crate::core::Index::new();
    repo.save_index(&empty_index)?;

    println!("Committed {} files", index.entries.len());
    println!("Commit hash: {}", commit.short_hash());
    Ok(())
}