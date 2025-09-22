use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod core;
mod utils;

use commands::*;

#[derive(Parser)]
#[command(name = "nvcs")]
#[command(about = "A naive version control system")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init,
    /// Add files to the staging area
    Add {
        /// Files to add
        files: Vec<String>,
        /// Add all files
        #[arg(short, long)]
        all: bool,
    },
    /// Commit staged changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
        /// Author name
        #[arg(short, long)]
        author: Option<String>,
    },
    /// Show repository status
    Status,
    /// Show commit history
    Log {
        /// Number of commits to show
        #[arg(short, long)]
        count: Option<usize>,
        /// Show one line per commit
        #[arg(long)]
        oneline: bool,
    },
    /// Show differences
    Diff {
        /// Show staged changes
        #[arg(long)]
        staged: bool,
        /// Compare specific files
        files: Vec<String>,
    },
    /// Create or switch branches
    Branch {
        /// Branch name
        name: Option<String>,
        /// List all branches
        #[arg(short, long)]
        list: bool,
        /// Delete branch
        #[arg(short, long)]
        delete: Option<String>,
    },
    /// Switch branches or restore files
    Checkout {
        /// Branch name or commit hash
        target: String,
        /// Create new branch
        #[arg(short, long)]
        branch: bool,
    },
    /// Merge branches
    Merge {
        /// Branch to merge
        branch: String,
    },
    /// Show commit details
    Show {
        /// Commit hash (defaults to HEAD)
        commit: Option<String>,
    },
    /// Reset changes
    Reset {
        /// Reset mode (soft, mixed, hard)
        #[arg(long, default_value = "mixed")]
        mode: String,
        /// Target commit
        target: Option<String>,
    },
    /// Remove files from working tree and index
    Rm {
        /// Files to remove
        files: Vec<String>,
        /// Remove from index only
        #[arg(long)]
        cached: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::execute(),
        Commands::Add { files, all } => add::execute(files, all),
        Commands::Commit { message, author } => commit::execute(message, author),
        Commands::Status => status::execute(),
        Commands::Log { count, oneline } => log::execute(count, oneline),
        Commands::Diff { staged, files } => diff::execute(staged, files),
        Commands::Branch { name, list, delete } => branch::execute(name, list, delete),
        Commands::Checkout { target, branch } => checkout::execute(target, branch),
        Commands::Merge { branch } => merge::execute(branch),
        Commands::Show { commit } => show::execute(commit),
        Commands::Reset { mode, target } => reset::execute(mode, target),
        Commands::Rm { files, cached } => rm::execute(files, cached),
    }
}