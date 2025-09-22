use std::fs;
use anyhow::Result;
use crate::core::Repository;
use crate::utils::diff::print_diff;

pub fn execute(staged: bool, files: Vec<String>) -> Result<()> {
    let repo = Repository::find_repository()?;
    let index = repo.load_index()?;

    if staged {
        // Show diff between HEAD and staged files
        let head_commit = match repo.get_head()? {
            Some(commit) => commit,
            None => {
                println!("No commits yet - showing all staged files as new");
                for (path, _) in &index.entries {
                    let file_path = repo.root.join(path);
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        print_diff("", &content, &path.to_string_lossy());
                    }
                }
                return Ok(());
            }
        };

        let commit_obj = repo.load_object(&head_commit)?;
        let commit = commit_obj.as_commit().unwrap();
        let tree_obj = repo.load_object(&commit.tree)?;
        let tree = tree_obj.as_tree().unwrap();

        for (path, entry) in &index.entries {
            let file_path = repo.root.join(path);
            let new_content = fs::read_to_string(&file_path).unwrap_or_default();
            
            // Find corresponding file in HEAD tree
            let path_str = path.to_string_lossy();
            if let Some(tree_entry) = tree.entries.get(&path_str.to_string()) {
                let blob_obj = repo.load_object(&tree_entry.hash)?;
                let blob = blob_obj.as_blob().unwrap();
                let old_content = blob.to_string().unwrap_or_default();
                
                if old_content != new_content {
                    print_diff(&old_content, &new_content, &path_str);
                }
            } else {
                // New file
                print_diff("", &new_content, &path_str);
            }
        }
    } else {
        // Show diff between staged and working directory
        if files.is_empty() {
            // Show all modified files
            for (path, entry) in &index.entries {
                let file_path = repo.root.join(path);
                if let Ok(current_content) = fs::read_to_string(&file_path) {
                    // Get staged content
                    let blob_obj = repo.load_object(&entry.hash)?;
                    let blob = blob_obj.as_blob().unwrap();
                    let staged_content = blob.to_string().unwrap_or_default();
                    
                    if staged_content != current_content {
                        print_diff(&staged_content, &current_content, &path.to_string_lossy());
                    }
                }
            }
        } else {
            // Show diff for specific files
            for file_str in files {
                let path = std::path::Path::new(&file_str);
                if let Some(entry) = index.entries.get(path) {
                    let file_path = repo.root.join(path);
                    if let Ok(current_content) = fs::read_to_string(&file_path) {
                        let blob_obj = repo.load_object(&entry.hash)?;
                        let blob = blob_obj.as_blob().unwrap();
                        let staged_content = blob.to_string().unwrap_or_default();
                        
                        print_diff(&staged_content, &current_content, &file_str);
                    }
                } else {
                    println!("File not staged: {}", file_str);
                }
            }
        }
    }

    Ok(())
}