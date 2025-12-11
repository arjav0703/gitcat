use anyhow::Result;
use std::process::Command;

mod status;
use status::Status;

pub struct Git {}

impl Git {
    pub fn repo_check() -> Result<bool> {
        use std::fs;
        let is_repo = fs::metadata(".git").is_ok();
        Ok(is_repo)
    }

    pub async fn status() -> Result<Status> {
        let output = Command::new("git").arg("status").output()?;
        let status = Status::from_str(&String::from_utf8_lossy(&output.stdout));
        Ok(status)
    }
}
