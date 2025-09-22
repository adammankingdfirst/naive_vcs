use std::fs;
use anyhow::Result;
use crate::core::Repository;

pub fn execute(target: String, create_branch: bool) -> Result<()> {
    let repo = Repository::find_repository()?;

    if create_branch {
        // Create and checkout new branch
        let head_commit = repo.get_head()?
            .ok_or_else(|| anyhow::anyhow!("No commits yet - cannot create branch"))?;
        
        repo.create_branch(&target, &head_commit)?;
        repo.checkout_branch(&target)?;
        println!("Switched to a new branch '{}'", target);
    } else {
        // Check if target is a branch name
        let branches = repo.list_branches()?;
        if branches.contains(&target) {
            repo.checkout_branch(&target)?;
            println!("Switched to branch '{}'", target);
        } else {
            // Try to checkout specific commit
            if repo.load_object(&target).is_ok() {
                fs::write(&repo.head_file, format!("{}\n", target))?;
                println!("HEAD is now at {} (detached)", &target[..8]);
            } else {
                return Err(anyhow::anyhow!("Branch or commit '{}' not found", target));
            }
        }
    }

    // TODO: Update working directory files to match the checked out commit
    // This would involve reading the tree from the target commit and updating files
    
    Ok(())
}