use anyhow::Result;
use crate::core::Repository;

pub fn execute() -> Result<()> {
    let repo = Repository::new(".");
    repo.init()
}