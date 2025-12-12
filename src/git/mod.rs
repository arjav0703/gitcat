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

    pub async fn commit(args: &[String]) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.arg("commit");
        for arg in args {
            cmd.arg(arg);
        }
        let output = cmd.output()?;
        if output.status.success() {
            println!("😺 Your changes are ready to be sent to meowland!");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else if String::from_utf8_lossy(&output.stdout).contains("nothing to commit") {
            println!("😺 Nothing to commit! Your code is already purrfect!");
        } else {
            println!(
                "🐾 Commit failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn push(args: &[String]) -> Result<()> {
        let output = Command::new("git").arg("push").args(args).output()?;
        if output.status.success() {
            println!("🚀 Pushed your beautiful code to meowland!");
        } else {
            println!(
                "🐾 Journey to meowland failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn pull(args: &[String]) -> Result<()> {
        let output = Command::new("git").arg("pull").args(args).output()?;
        if output.status.success() {
            println!("⬇️ Fetched the latest meow updates from meowland!");
        } else {
            println!(
                "🐾 Pulling updates from meowland failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }
}
