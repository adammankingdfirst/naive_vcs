use std::fs;
use std::collections::HashSet;
use anyhow::Result;
use colored::*;
use crate::core::Repository;
use crate::utils::files::{find_files_in_directory, is_ignored};
use crate::utils::hash::calculate_file_hash;

pub fn execute() -> Result<()> {
    let repo = Repository::find_repository()?;
    let index = repo.load_index()?;

    // Show current branch
    if let Some(branch) = repo.get_current_branch()? {
        println!("On branch {}", branch.bright_cyan());
    } else {
        println!("HEAD detached");
    }

    // Get all files in working directory
    let all_files = find_files_in_directory(&repo.root)?;
    let mut working_files = HashSet::new();
    
    for file_path in all_files {
        if !is_ignored(&file_path) {
            if let Ok(relative_path) = file_path.strip_prefix(&repo.root) {
                working_files.insert(relative_path.to_path_buf());
            }
        }
    }

    // Staged files
    let staged_files: HashSet<_> = index.entries.keys().cloned().collect();
    
    // Modified files (staged but changed in working directory)
    let mut modified_files = Vec::new();
    for (path, entry) in &index.entries {
        let full_path = repo.root.join(path);
        if full_path.exists() {
            if let Ok(current_hash) = calculate_file_hash(&full_path) {
                if current_hash != entry.hash {
                    modified_files.push(path);
                }
            }
        }
    }

    // Untracked files
    let untracked_files: Vec<_> = working_files
        .difference(&staged_files)
        .collect();

    // Deleted files (staged but not in working directory)
    let deleted_files: Vec<_> = staged_files
        .difference(&working_files)
        .collect();

    // Display status
    if !staged_files.is_empty() {
        println!("\nChanges to be committed:");
        println!("  (use \"nvcs reset HEAD <file>...\" to unstage)");
        for file in &staged_files {
            if deleted_files.contains(&file) {
                println!("        {}: {}", "deleted".red(), file.display());
            } else {
                println!("        {}: {}", "new file".green(), file.display());
            }
        }
    }

    if !modified_files.is_empty() {
        println!("\nChanges not staged for commit:");
        println!("  (use \"nvcs add <file>...\" to update what will be committed)");
        for file in modified_files {
            println!("        {}: {}", "modified".yellow(), file.display());
        }
    }

    if !untracked_files.is_empty() {
        println!("\nUntracked files:");
        println!("  (use \"nvcs add <file>...\" to include in what will be committed)");
        for file in untracked_files {
            println!("        {}", file.display().to_string().red());
        }
    }

    if staged_files.is_empty() && modified_files.is_empty() && untracked_files.is_empty() {
        println!("\nNothing to commit, working tree clean");
    }

    Ok(())
}