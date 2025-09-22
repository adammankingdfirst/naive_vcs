use std::fs;
use anyhow::Result;
use crate::core::{Repository, Index};

pub fn execute(mode: String, target: Option<String>) -> Result<()> {
    let repo = Repository::find_repository()?;
    
    let target_commit = match target {
        Some(t) => t,
        None => repo.get_head()?.ok_or_else(|| anyhow::anyhow!("No commits yet"))?,
    };

    // Verify target commit exists
    repo.load_object(&target_commit)?;

    match mode.as_str() {
        "soft" => {
            // Only move HEAD, keep index and working directory
            if let Some(current_branch) = repo.get_current_branch()? {
                repo.update_ref(&format!("refs/heads/{}", current_branch), &target_commit)?;
            } else {
                fs::write(&repo.head_file, format!("{}\n", target_commit))?;
            }
            println!("Soft reset to {}", &target_commit[..8]);
        }
        "mixed" => {
            // Move HEAD and reset index, keep working directory
            if let Some(current_branch) = repo.get_current_branch()? {
                repo.update_ref(&format!("refs/heads/{}", current_branch), &target_commit)?;
            } else {
                fs::write(&repo.head_file, format!("{}\n", target_commit))?;
            }
            
            // Clear index
            let empty_index = Index::new();
            repo.save_index(&empty_index)?;
            
            println!("Mixed reset to {}", &target_commit[..8]);
        }
        "hard" => {
            // Move HEAD, reset index, and reset working directory
            if let Some(current_branch) = repo.get_current_branch()? {
                repo.update_ref(&format!("refs/heads/{}", current_branch), &target_commit)?;
            } else {
                fs::write(&repo.head_file, format!("{}\n", target_commit))?;
            }
            
            // Clear index
            let empty_index = Index::new();
            repo.save_index(&empty_index)?;
            
            // TODO: Reset working directory files to match target commit
            // This would involve reading the tree from target commit and updating files
            
            println!("Hard reset to {}", &target_commit[..8]);
            println!("Warning: Working directory changes not implemented yet");
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid reset mode: {}. Use soft, mixed, or hard", mode));
        }
    }

    Ok(())
}