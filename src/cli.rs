use clap::{Command, arg};

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
}
