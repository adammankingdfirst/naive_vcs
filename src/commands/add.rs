use std::path::Path;
use anyhow::Result;
use crate::core::Repository;
use crate::utils::files::{find_files_in_directory, is_ignored};

pub fn execute(files: Vec<String>, all: bool) -> Result<()> {
    let repo = Repository::find_repository()?;
    let mut index = repo.load_index()?;

    if all {
        // Add all files in the repository
        let all_files = find_files_in_directory(&repo.root)?;
        for file_path in all_files {
            if !is_ignored(&file_path) {
                match index.add_file(&file_path, &repo.root) {
                    Ok(_) => {
                        let relative_path = file_path.strip_prefix(&repo.root)?;
                        println!("Added: {}", relative_path.display());
                    }
                    Err(e) => eprintln!("Warning: Could not add {}: {}", file_path.display(), e),
                }
            }
        }
    } else if files.is_empty() {
        return Err(anyhow::anyhow!("No files specified. Use --all to add all files."));
    } else {
        // Add specific files
        for file_str in files {
            let file_path = Path::new(&file_str);
            let full_path = if file_path.is_absolute() {
                file_path.to_path_buf()
            } else {
                repo.root.join(file_path)
            };

            if !full_path.exists() {
                eprintln!("Warning: File does not exist: {}", file_str);
                continue;
            }

            if is_ignored(&full_path) {
                eprintln!("Warning: File is ignored: {}", file_str);
                continue;
            }

            match index.add_file(&full_path, &repo.root) {
                Ok(_) => println!("Added: {}", file_str),
                Err(e) => eprintln!("Error adding {}: {}", file_str, e),
            }
        }
    }

    repo.save_index(&index)?;
    Ok(())
}