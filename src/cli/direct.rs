use clap::{Arg, Command};

use crate::Configuration;

pub fn run() -> Command {
    Command::new("git-message-ai")
        .about("A CLI to generate your git messages!")
        .author("codermarcos")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generates a message with predefined arguments")
                .long_about(
                    "This command generates a message with predefined arguments. The ticket id is not required.",
                )
                .short_flag('g')
                .long_flag("generate")
                .args_conflicts_with_subcommands(true)
                .arg(
                    Arg::new("ticket-id")
                        .short('t')
                        .long("ticket-id")
                        .value_name("DP-42")
                        .required(false)
                        .default_value("")
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
        .unwrap()
        .to_string()
}
