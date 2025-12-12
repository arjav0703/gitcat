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
            let args: Vec<String> = sub_m
                .get_many::<String>("ARGS")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
            Git::commit(&args).await?;
        }
        Some(("push", args)) => {
            let args: Vec<String> = args
                .get_many::<String>("ARGS")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
            Git::push(&args).await?;
        }
        Some(("pull", args)) => {
            let args: Vec<String> = args
                .get_many::<String>("ARGS")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
            Git::pull(&args).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
