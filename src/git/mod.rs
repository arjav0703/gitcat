use anyhow::Result;
use std::{io::stdout, process::Command};

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
            if String::from_utf8_lossy(&output.stdout).contains("Everything up-to-date") {
                println!("😺 Everything is already up-to-date in meowland!");
                return Ok(());
            }

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

    pub async fn checkout(branch: &str) -> Result<()> {
        let output = Command::new("git").arg("checkout").arg(branch).output()?;
        if output.status.success() {
            println!("😺 Pounced to branch '{}'", branch);
        } else {
            println!(
                "🐾 Pouncing to branch '{}' failed:( {}",
                branch,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn create_branch(branch: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch)
            .output()?;
        if output.status.success() {
            println!("😺 Created and pounced to new branch '{}'", branch);
        } else {
            println!(
                "🐾 Creating branch '{}' failed:( {}",
                branch,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn diff(args: &[String]) -> Result<()> {
        Command::new("git")
            .arg("diff")
            .args(args)
            .stdout(stdout())
            .output()?;
        Ok(())
    }

    pub async fn stash() -> Result<()> {
        let output = Command::new("git").arg("stash").output()?;
        if output.status.success() {
            println!("Your changes will be safe in my dreams 😴💤!");
        } else {
            println!(
                "🐾 Stashing changes failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn unstash() -> Result<()> {
        let output = Command::new("git").arg("stash").arg("pop").output()?;
        if output.status.success() {
            println!("Welcome back to reality! Your changes are restored 😺!");
        } else {
            println!(
                "🐾 Unstashing changes failed:( {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub async fn show_stash_list() -> Result<()> {
        Command::new("git")
            .arg("stash")
            .arg("list")
            .stdout(stdout())
            .output()?;
        Ok(())
    }
}
