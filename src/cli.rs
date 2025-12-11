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
        .subcommand(Command::new("push").about("Push commits to the default remote repository"))
        .subcommand(Command::new("pull").about("Pull updates from the default remote repository"))
}
