mod cli;
mod git;

use anyhow::Result;
use git::Git;

#[tokio::main]
async fn main() -> Result<()> {
    let is_repo = Git::repo_check()?;
    if !is_repo {
        println!("Not a git repository.");
        return Ok(());
    }

    let matches = cli::cli().get_matches();
    match matches.subcommand() {
        Some(("hru", _)) => {
            let status = Git::status().await?;
            println!("{}", status.to_meowssage());
        }
        Some(("meow", sub_m)) => {
            let message = sub_m.get_one::<String>("MESSAGE").unwrap();
            let output = std::process::Command::new("git")
                .args(["commit", "-am", message])
                .output()?;
            if output.status.success() {
                println!("😺 Committed with message: {}", message);
            } else {
                println!(
                    "🐾 Commit failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
