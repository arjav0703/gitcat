use clap::{Command, arg};

pub fn cli() -> Command {
    Command::new("gitcat")
        .about("A CLI tool to interact with Git repositories")
        .subcommand_required(true)
        .subcommand(Command::new("hru").about("Show the working tree status"))
}
