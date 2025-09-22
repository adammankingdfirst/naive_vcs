use anyhow::Result;
use colored::*;
use crate::core::Repository;

pub fn execute(count: Option<usize>, oneline: bool) -> Result<()> {
    let repo = Repository::find_repository()?;
    
    let mut current_commit = match repo.get_head()? {
        Some(commit) => commit,
        None => {
            println!("No commits yet");
            return Ok(());
        }
    };

    let max_count = count.unwrap_or(usize::MAX);
    let mut shown = 0;

    while shown < max_count {
        let commit_obj = repo.load_object(&current_commit)?;
        let commit = commit_obj.as_commit().unwrap();

        if oneline {
            println!(
                "{} {}",
                commit.short_hash().yellow(),
                commit.message.lines().next().unwrap_or("")
            );
        } else {
            println!("{} {}", "commit".yellow(), current_commit);
            println!("{} {}", "Author:".bright_white(), commit.author);
            println!("{} {}", "Date:".bright_white(), commit.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
            println!();
            for line in commit.message.lines() {
                println!("    {}", line);
            }
            println!();
        }

        shown += 1;

        // Move to parent commit
        if let Some(parent) = commit.parents.first() {
            current_commit = parent.clone();
        } else {
            break;
        }
    }

    Ok(())
}