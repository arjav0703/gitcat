use anyhow::Result;
use std::{fs, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    let is_repo = Git::repo_check()?;
    if !is_repo {
        println!("Not a git repository.");
        return Ok(());
    }

    Git::status().await?;
    Ok(())
}

struct Git {}

impl Git {
    fn repo_check() -> Result<(bool)> {
        use std::fs;
        let is_repo = fs::metadata(".git").is_ok();
        Ok(is_repo)
    }

    async fn status() -> Result<()> {
        let output = Command::new("git").arg("status").output()?;
        dbg!(output);
        Ok(())
    }
}
