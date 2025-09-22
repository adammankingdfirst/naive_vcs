use anyhow::Result;
use colored::*;
use crate::core::Repository;

pub fn execute(commit_hash: Option<String>) -> Result<()> {
    let repo = Repository::find_repository()?;
    
    let commit_hash = match commit_hash {
        Some(hash) => hash,
        None => repo.get_head()?.ok_or_else(|| anyhow::anyhow!("No commits yet"))?,
    };

    let commit_obj = repo.load_object(&commit_hash)?;
    let commit = commit_obj.as_commit()
        .ok_or_else(|| anyhow::anyhow!("Object is not a commit"))?;

    // Show commit info
    println!("{} {}", "commit".yellow(), commit_hash);
    if commit.is_merge() {
        println!("{} {}", "Merge:".bright_white(), commit.parents.join(" "));
    }
    println!("{} {}", "Author:".bright_white(), commit.author);
    println!("{} {}", "Date:".bright_white(), commit.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    println!();
    for line in commit.message.lines() {
        println!("    {}", line);
    }
    println!();

    // Show diff from parent (simplified)
    if let Some(parent_hash) = commit.parents.first() {
        let parent_obj = repo.load_object(parent_hash)?;
        let parent_commit = parent_obj.as_commit().unwrap();
        
        let current_tree = repo.load_object(&commit.tree)?;
        let parent_tree = repo.load_object(&parent_commit.tree)?;
        
        let current_tree = current_tree.as_tree().unwrap();
        let parent_tree = parent_tree.as_tree().unwrap();
        
        // Simple diff showing changed files
        for (path, entry) in &current_tree.entries {
            if let Some(parent_entry) = parent_tree.entries.get(path) {
                if entry.hash != parent_entry.hash {
                    println!("{} {}", "modified:".yellow(), path);
                }
            } else {
                println!("{} {}", "new file:".green(), path);
            }
        }
        
        for (path, _) in &parent_tree.entries {
            if !current_tree.entries.contains_key(path) {
                println!("{} {}", "deleted:".red(), path);
            }
        }
    } else {
        // Initial commit - show all files as new
        let tree_obj = repo.load_object(&commit.tree)?;
        let tree = tree_obj.as_tree().unwrap();
        
        for (path, _) in &tree.entries {
            println!("{} {}", "new file:".green(), path);
        }
    }

    Ok(())
}