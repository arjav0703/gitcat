use clap::{ArgMatches, Command, arg};

pub trait ArgMatchesExt {
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

pub fn cli() -> Command {
    Command::new("gitcat")
        .about("A CLI tool to interact with Git repositories")
        .subcommand_required(true)
        .subcommand(Command::new("hru").about("Show the working tree status"))
        .subcommand(
            Command::new("meow")
                .about("Commit changes with a cat-themed message")
                .arg(
                    arg!([ARGS] ... "Git commit arguments (e.g., -am 'message', --amend)")
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
        )
        .subcommand(
            Command::new("touch")
                .about("Add files to the staging area")
                .arg(
                    arg!([ARGS] ... "Git add arguments (e.g., ., file.txt)")
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("Push commits to the default remote repository")
                .arg(
                    arg!([ARGS] ... "Git push arguments (e.g., origin main, --force)")
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull updates from the default remote repository")
                .arg(
                    arg!([ARGS] ... "Git pull arguments (e.g., origin main, --rebase)")
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
        )
        .subcommand(
            Command::new("pounce")
                .about("Checkout to the specified branch.")
                .arg(
                    arg!([BRANCH] "The branch to checkout to")
                        .required(true)
                        .allow_hyphen_values(true),
                ),
        )
        .subcommand(
            Command::new("scratch")
                .about("Create a new branch (and checkout to it)")
                .arg(
                    arg!([BRANCH] "The name of the new branch")
                        .required(true)
                        .allow_hyphen_values(true),
                ),
        )
        .subcommand(
            Command::new("sniff").about("Show the git diff").arg(
                arg!([ARGS] ... "Git diff arguments (e.g., HEAD~1, --stat)")
                    .allow_hyphen_values(true)
                    .trailing_var_arg(true),
            ),
        )
        .subcommand(Command::new("nap").about("Stash uncommitted changes"))
        .subcommand(Command::new("wake").about("Apply stashed changes"))
        .subcommand(Command::new("dreams").about("Show the list of stashed changes"))
        .subcommand(
            Command::new("mood").about("Set the cat mood ").arg(
                arg!([MOOD] "Available moods: chaotic")
                    .required(true)
                    .allow_hyphen_values(true),
            ),
        )
}
