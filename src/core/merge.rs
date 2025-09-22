use std::collections::HashMap;
use anyhow::Result;
use crate::core::{Repository, Object, Commit, Tree};

pub struct MergeResult {
    pub success: bool,
    pub conflicts: Vec<String>,
    pub merged_tree: Option<String>,
}

pub fn merge_commits(
    repo: &Repository,
    base_commit: &str,
    our_commit: &str,
    their_commit: &str,
) -> Result<MergeResult> {
    let base_obj = repo.load_object(base_commit)?;
    let our_obj = repo.load_object(our_commit)?;
    let their_obj = repo.load_object(their_commit)?;

    let base_commit = base_obj.as_commit().unwrap();
    let our_commit = our_obj.as_commit().unwrap();
    let their_commit = their_obj.as_commit().unwrap();

    let base_tree = repo.load_object(&base_commit.tree)?;
    let our_tree = repo.load_object(&our_commit.tree)?;
    let their_tree = repo.load_object(&their_commit.tree)?;

    merge_trees(repo, base_tree.as_tree().unwrap(), our_tree.as_tree().unwrap(), their_tree.as_tree().unwrap())
}

fn merge_trees(
    repo: &Repository,
    base_tree: &Tree,
    our_tree: &Tree,
    their_tree: &Tree,
) -> Result<MergeResult> {
    let mut merged_tree = Tree::new();
    let mut conflicts = Vec::new();

    // Collect all file paths from all trees
    let mut all_paths = std::collections::HashSet::new();
    for entry in base_tree.entries.keys() {
        all_paths.insert(entry.clone());
    }
    for entry in our_tree.entries.keys() {
        all_paths.insert(entry.clone());
    }
    for entry in their_tree.entries.keys() {
        all_paths.insert(entry.clone());
    }

    for path in all_paths {
        let base_entry = base_tree.entries.get(&path);
        let our_entry = our_tree.entries.get(&path);
        let their_entry = their_tree.entries.get(&path);

        match (base_entry, our_entry, their_entry) {
            // File unchanged in both branches
            (Some(base), Some(our), Some(their)) if our.hash == base.hash && their.hash == base.hash => {
                merged_tree.add_entry(path, base.hash.clone(), base.is_file);
            }
            // File changed only in our branch
            (Some(_), Some(our), Some(their)) if their.hash == base_entry.unwrap().hash => {
                merged_tree.add_entry(path, our.hash.clone(), our.is_file);
            }
            // File changed only in their branch
            (Some(_), Some(our), Some(their)) if our.hash == base_entry.unwrap().hash => {
                merged_tree.add_entry(path, their.hash.clone(), their.is_file);
            }
            // File changed in both branches - conflict
            (Some(_), Some(our), Some(their)) if our.hash != their.hash => {
                conflicts.push(path.clone());
                // For now, take our version
                merged_tree.add_entry(path, our.hash.clone(), our.is_file);
            }
            // File added in our branch only
            (None, Some(our), None) => {
                merged_tree.add_entry(path, our.hash.clone(), our.is_file);
            }
            // File added in their branch only
            (None, None, Some(their)) => {
                merged_tree.add_entry(path, their.hash.clone(), their.is_file);
            }
            // File deleted in our branch
            (Some(_), None, Some(their)) => {
                // Conflict: deleted vs modified
                conflicts.push(format!("{} (deleted vs modified)", path));
            }
            // File deleted in their branch
            (Some(_), Some(our), None) => {
                // Conflict: modified vs deleted
                conflicts.push(format!("{} (modified vs deleted)", path));
            }
            _ => {
                // Other cases - take our version if available, otherwise their version
                if let Some(our) = our_entry {
                    merged_tree.add_entry(path, our.hash.clone(), our.is_file);
                } else if let Some(their) = their_entry {
                    merged_tree.add_entry(path, their.hash.clone(), their.is_file);
                }
            }
        }
    }

    let success = conflicts.is_empty();
    let merged_tree_hash = if success {
        let tree_obj = Object::Tree(merged_tree);
        Some(repo.store_object(&tree_obj)?)
    } else {
        None
    };

    Ok(MergeResult {
        success,
        conflicts,
        merged_tree: merged_tree_hash,
    })
}