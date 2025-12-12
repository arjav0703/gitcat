mod cli;
mod git;

use anyhow::Result;
use clap::ArgMatches;
use git::Git;

trait ArgMatchesExt {
    fn get_args(&self) -> Vec<String>;
}

impl ArgMatchesExt for ArgMatches {
    fn get_args(&self) -> Vec<String> {
        self.get_many::<String>("ARGS")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect()
    }
}

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
            Git::commit(&sub_m.get_args()).await?;
        }
        Some(("push", args)) => {
            Git::push(&args.get_args()).await?;
        }
        Some(("pull", args)) => {
            Git::pull(&args.get_args()).await?;
        }
        Some(("pounce", sub_m)) => {
            let branch = sub_m
                .get_one::<String>("BRANCH")
                .expect("Branch argument is required");
            Git::checkout(branch).await?;
        }
        Some(("scratch", sub_m)) => {
            let branch = sub_m
                .get_one::<String>("BRANCH")
                .expect("Branch argument is required");
            Git::create_branch(branch).await?;
        }
        Some(("sniff", args)) => {
            Git::diff(&args.get_args()).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
