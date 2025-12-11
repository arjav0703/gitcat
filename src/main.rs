use anyhow::Result;
use std::{fs, process::Command};

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

#[derive(Debug)]
enum Status {
    Clean,
    Unstaged,
    Staged,
}

impl Status {
    fn from_str(s: &str) -> Self {
        if s.contains("nothing to commit, working tree clean") {
            Status::Clean
        } else if s.contains("Changes not staged for commit") {
            Status::Unstaged
        } else if s.contains("Changes to be committed") {
            Status::Staged
        } else {
            Status::Clean
        }
    }
}

struct Git {}

impl Git {
    fn repo_check() -> Result<bool> {
        use std::fs;
        let is_repo = fs::metadata(".git").is_ok();
        Ok(is_repo)
    }

    async fn status() -> Result<Status> {
        let output = Command::new("git").arg("status").output()?;
        let status = Status::from_str(&String::from_utf8_lossy(&output.stdout));
        Ok(status)
    }
}
