use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use ignore::WalkBuilder;
use anyhow::Result;

pub fn find_files_in_directory<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in WalkBuilder::new(dir)
        .hidden(false)
        .git_ignore(true)
        .build()
    {
        let entry = entry?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            files.push(entry.path().to_path_buf());
        }
    }
    
    Ok(files)
}

pub fn is_ignored(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    
    // Ignore .nvcs directory
    if path_str.contains("/.nvcs/") || path_str.starts_with(".nvcs/") {
        return true;
    }
    
    // Ignore common patterns
    let ignore_patterns = [
        ".git/", ".svn/", ".hg/",
        "target/", "build/", "dist/",
        "node_modules/", ".vscode/", ".idea/",
        "*.tmp", "*.log", "*.swp", "*.swo",
        ".DS_Store", "Thumbs.db",
    ];
    
    for pattern in &ignore_patterns {
        if pattern.ends_with('/') {
            if path_str.contains(pattern) {
                return true;
            }
        } else if pattern.contains('*') {
            // Simple glob matching
            let pattern = pattern.replace('*', "");
            if path_str.ends_with(&pattern) {
                return true;
            }
        } else if path_str.contains(pattern) {
            return true;
        }
    }
    
    false
}

pub fn get_relative_path<P: AsRef<Path>>(path: P, base: P) -> Result<PathBuf> {
    let path = path.as_ref().canonicalize()?;
    let base = base.as_ref().canonicalize()?;
    Ok(path.strip_prefix(base)?.to_path_buf())
}