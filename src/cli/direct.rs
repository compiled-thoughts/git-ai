use clap::{Arg, Command};

use crate::Configuration;

pub fn run() -> Command {
    Command::new("git-ai")
        .about("A CLI to generate messages for commits, title and description for pull requests based on task using AI!")
        .author("codermarcos")
        .subcommand(
            Command::new("generate")
                .about("Generate a commit message")
                .long_about(
                    "This command generates a commit message based on a task and code changes.",
                )
                .short_flag('g')
                .long_flag("generate")
                .arg_required_else_help(true)
                .args_conflicts_with_subcommands(true)
                .arg(
                    Arg::new("ticket-id")
                        .short('t')
                        .long("ticket-id")
                        .value_name("RT-666")
                        .required(true)
                        .help("What is the ticket id")
                        .long_help(
                            "This ticket will be read to improve the commit message",
                        ),
                )
                .subcommand(
                    Command::new("interactive")
                        .short_flag('i')
                        .long_flag("interactive")
                        .about("Use the interactive mode")
                        .long_about(
                            "When this flag is enabled you can add the ticket interactively"
                        )
                )
        )
        .subcommand(
            Command::new("create")
                .about("Create a pull request with title and description based on a ticket or task")
                .long_about(
                    "This command creates a pull request into the configured vcs with a genereted title and description based on a task and code changes.",
                )
                .short_flag('c')
                .long_flag("create")
                .arg_required_else_help(true)
                .args_conflicts_with_subcommands(true)
                .arg(
                    Arg::new("ticket-id")
                        .short('t')
                        .long("ticket-id")
                        .value_name("RT-666")
                        .required(true)
                        .help("What is the ticket id")
                        .long_help(
                            "This ticket will be read to improve the commit message",
                        ),
                )
                .arg(
                    Arg::new("branch-source")
                        .short('s')
                        .long("branch-source")
                        .value_name("feat/RT-000/add-humans")
                        .help("What is the name source branch name")
                        .long_help(
                            "The name of the branch which you will merge into the target branch. If you omit it will be the current branch.",
                        ),
                )
                .arg(
                    Arg::new("branch-target")
                        .short('t')
                        .long("branch-target")
                        .value_name("feat/RT-666/remove-humans")
                        .required(true)
                        .help("What is the name target branch name")
                        .long_help(
                            "The name of the branch which will receives the merge",
                        ),
                )
                .subcommand(
                    Command::new("interactive")
                        .short_flag('i')
                        .long_flag("interactive")
                        .about("Use the interactive mode")
                        .long_about(
                            "When this flag is enabled you can add the ticket interactively"
                        )
                )
        )
        .subcommand(
            Command::new("initiate")
                .about("Initializes the configuration")
                .short_flag('i')
                .long_flag("initiate")
                .long_about("This command initializes the configuration"),
        )
}

pub fn generate(sub_matches: &clap::ArgMatches, _configuration: &Configuration) -> String {
    sub_matches
        .get_one::<String>("ticket-id")
        .expect("ticket-id is required")
        .to_string()
}
