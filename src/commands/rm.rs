use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::core::Repository;

pub fn execute(files: Vec<String>, cached: bool) -> Result<()> {
    let repo = Repository::find_repository()?;
    let mut index = repo.load_index()?;

    if files.is_empty() {
        return Err(anyhow::anyhow!("No files specified"));
    }

    for file_str in files {
        let file_path = Path::new(&file_str);
        let relative_path = if file_path.is_absolute() {
            file_path.strip_prefix(&repo.root)?
        } else {
            file_path
        };

        if !index.is_staged(relative_path) {
            eprintln!("Warning: File not staged: {}", file_str);
            continue;
        }

        // Remove from index
        index.remove_file(relative_path);
        println!("Removed from index: {}", file_str);

        // Remove from working directory if not --cached
        if !cached {
            let full_path = repo.root.join(relative_path);
            if full_path.exists() {
                fs::remove_file(&full_path)?;
                println!("Removed from working directory: {}", file_str);
            }
        }
    }

    repo.save_index(&index)?;
    Ok(())
}