mod cli;
mod command;
mod config;
mod error;
mod git;

use cli::ArgMatchesExt;
use error::Result;
use git::GitRepository;

#[tokio::main]
async fn main() -> Result<()> {
    if !GitRepository::is_repository()? {
        eprintln!("Not a git repository.");
        std::process::exit(1);
    }

    let mut repo = GitRepository::new();

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("hru", _)) => {
            repo.display_status().await?;
        }
        Some(("meow", sub_m)) => {
            repo.commit(&sub_m.get_args()).await?;
        }
        Some(("touch", args)) => {
            repo.add(&args.get_args()).await?;
        }
        Some(("push", args)) => {
            repo.push(&args.get_args()).await?;
        }
        Some(("pull", args)) => {
            repo.pull(&args.get_args()).await?;
        }
        Some(("pounce", sub_m)) => {
            let branch = sub_m
                .get_one::<String>("BRANCH")
                .ok_or_else(|| error::GitCatError::MissingArgument("BRANCH".to_string()))?;
            repo.checkout(branch).await?;
        }
        Some(("scratch", sub_m)) => {
            let branch = sub_m
                .get_one::<String>("BRANCH")
                .ok_or_else(|| error::GitCatError::MissingArgument("BRANCH".to_string()))?;
            repo.create_branch(branch).await?;
        }
        Some(("sniff", args)) => {
            repo.diff(&args.get_args()).await?;
        }
        Some(("nap", _)) => {
            repo.stash().await?;
        }
        Some(("wake", _)) => {
            repo.unstash().await?;
        }
        Some(("dreams", _)) => {
            repo.show_stash_list().await?;
        }
        Some(("mood", mood)) => {
            let mood_value = mood
                .get_one::<String>("MOOD")
                .ok_or_else(|| error::GitCatError::MissingArgument("MOOD".to_string()))?;
            let _ = repo.set_mood(mood_value).await;
        }
        Some(("cook", directory)) => {
            let dir_value = directory
                .get_one::<String>("PATH")
                .ok_or_else(|| error::GitCatError::MissingArgument("PATH".to_string()))?;
            repo.create_repository(dir_value).await?;
        }
        _ => unreachable!("Unhandled subcommand"),
    }

    Ok(())
}
