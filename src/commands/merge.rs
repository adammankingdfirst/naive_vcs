use std::fs;
use anyhow::Result;
use crate::core::{Repository, Object, Commit};
use crate::core::merge::merge_commits;

pub fn execute(branch_name: String) -> Result<()> {
    let repo = Repository::find_repository()?;
    
    // Get current branch and commit
    let current_branch = repo.get_current_branch()?
        .ok_or_else(|| anyhow::anyhow!("Not on a branch - cannot merge"))?;
    
    let current_commit = repo.get_head()?
        .ok_or_else(|| anyhow::anyhow!("No commits yet"))?;

    // Get target branch commit
    let branch_file = repo.refs_dir.join("heads").join(&branch_name);
    if !branch_file.exists() {
        return Err(anyhow::anyhow!("Branch '{}' does not exist", branch_name));
    }
    
    let target_commit = fs::read_to_string(&branch_file)?.trim().to_string();

    if current_commit == target_commit {
        println!("Already up to date.");
        return Ok(());
    }

    // Find common ancestor (simplified - just use current commit as base for now)
    let base_commit = current_commit.clone();

    // Perform merge
    let merge_result = merge_commits(&repo, &base_commit, &current_commit, &target_commit)?;

    if !merge_result.success {
        println!("Merge conflicts in:");
        for conflict in &merge_result.conflicts {
            println!("  {}", conflict);
        }
        println!("Fix conflicts and commit the result.");
        return Ok(());
    }

    // Create merge commit
    let tree_hash = merge_result.merged_tree.unwrap();
    let merge_commit = Commit::new(
        tree_hash,
        vec![current_commit, target_commit],
        "System".to_string(),
        format!("Merge branch '{}'", branch_name),
    );

    let commit_obj = Object::Commit(merge_commit.clone());
    let commit_hash = repo.store_object(&commit_obj)?;

    // Update current branch
    repo.update_ref(&format!("refs/heads/{}", current_branch), &commit_hash)?;

    println!("Merged branch '{}' into '{}'", branch_name, current_branch);
    println!("Merge commit: {}", merge_commit.short_hash());

    Ok(())
}