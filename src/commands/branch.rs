use anyhow::Result;
use colored::*;
use crate::core::Repository;

pub fn execute(name: Option<String>, list: bool, delete: Option<String>) -> Result<()> {
    let repo = Repository::find_repository()?;

    if let Some(branch_to_delete) = delete {
        repo.delete_branch(&branch_to_delete)?;
        println!("Deleted branch '{}'", branch_to_delete);
        return Ok(());
    }

    if list || name.is_none() {
        // List all branches
        let branches = repo.list_branches()?;
        let current_branch = repo.get_current_branch()?;

        if branches.is_empty() {
            println!("No branches found");
            return Ok(());
        }

        for branch in branches {
            if current_branch.as_deref() == Some(&branch) {
                println!("* {}", branch.green());
            } else {
                println!("  {}", branch);
            }
        }
    } else if let Some(branch_name) = name {
        // Create new branch
        let head_commit = repo.get_head()?
            .ok_or_else(|| anyhow::anyhow!("No commits yet - cannot create branch"))?;
        
        repo.create_branch(&branch_name, &head_commit)?;
        println!("Created branch '{}'", branch_name);
    }

    Ok(())
}