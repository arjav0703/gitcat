use anyhow::Result;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    Git::status().await?;
    Ok(())
}

struct Git {}

impl Git {
    async fn status() -> Result<()> {
        let output = Command::new("git").arg("status").output()?;
        dbg!(output);
        Ok(())
    }
}
