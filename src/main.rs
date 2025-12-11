use anyhow::Result;
use std::process::Command;

mod git;
use git::Git;

#[tokio::main]
async fn main() -> Result<()> {
    let is_repo = Git::repo_check()?;
    if !is_repo {
        println!("Not a git repository.");
        return Ok(());
    }

    let status = Git::status().await?;
    dbg!(status);
    Ok(())
}
