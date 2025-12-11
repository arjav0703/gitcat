use anyhow::Result;
use std::process::Command;

mod status;
use status::Status;

pub struct Git {}

impl Git {
    pub fn repo_check() -> Result<bool> {
        use std::fs;
        Ok(fs::metadata(".git").is_ok())
    }

    pub async fn status() -> Result<Status> {
        let output = Command::new("git").arg("status").output()?;
        Ok(Status::from_str(&String::from_utf8_lossy(&output.stdout)))
    }

    pub async fn commit(message: &str) -> Result<()> {
        Command::new("git").args(["add", "."]).output()?;
        let output = Command::new("git")
            .args(["commit", "-am", message])
            .output()?;
        if output.status.success() {
            println!("😺 Committed with message: {}", message);
        } else {
            println!(
                "🐾 Commit failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn push() -> Result<()> {
        let output = Command::new("git").arg("push").output()?;
        if output.status.success() {
            println!("🚀 Pushed your code to meowland!");
        } else {
            println!(
                "🐾 Journey to meowland failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }
}
