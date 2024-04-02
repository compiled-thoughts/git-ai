use clap::{Arg, Command};

pub fn run() -> Command {
    let dry_run_arg = || Arg::new("dry-run")
        .short('d')
        .long("dry-run")
        .required(false)
        .help("Just output the message")
        .long_help(
            "It will get the ticket and diff to generates the message using the chosed ai but not write the message",
        );

    let ticket_id_arg = || Arg::new("ticket-id")
        .short('t')
        .long("ticket-id")
        .value_name("DP-42")
        .required(false)
        .default_value("")
        .help("What is the ticket id")
        .long_help("This ticket will be read to improve the commit message");

    let interactive_subcommand = || Command::new("interactive")
        .short_flag('i')
        .long_flag("interactive")
        .about("Use the interactive mode")
        .long_about("When this flag is enabled you can add the inputs interactively")
        .arg(dry_run_arg());

    Command::new("git-ai")
        .about("A CLI to generate messages for commits, title and description for pull requests based on task using AI!")
        .author("codermarcos")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate a commit message")
                .long_about(
                    "This command generates a message with predefined arguments. The ticket id is not required.",
                )
                .short_flag('g')
                .long_flag("generate")
                .args_conflicts_with_subcommands(true)
                .arg(ticket_id_arg())
                .arg(dry_run_arg())
                .subcommand(interactive_subcommand())
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
                .arg(ticket_id_arg())
                .arg(
                    Arg::new("source-branch")
                        .short('s')
                        .long("source-branch")
                        .value_name("feat/RT-000/add-humans")
                        .help("What is the name source branch name")
                        .long_help(
                            "The name of the branch which you will merge into the target branch. If you omit it will be the current branch.",
                        ),
                )
                .arg(
                    Arg::new("target-branch")
                        .short('t')
                        .long("target-branch")
                        .value_name("feat/RT-666/remove-humans")
                        .help("What is the name target branch name")
                        .long_help(
                            "The name of the branch which will receives the merge. If you omit it will be the main branch.",
                        ),
                )
                .subcommand(interactive_subcommand())
        )
        .subcommand(
            Command::new("initiate")
                .about("Initializes the configuration")
                .short_flag('i')
                .long_flag("initiate")
                .long_about("This command initializes the configuration"),
        )
}

pub fn get_task_ids(sub_matches: &clap::ArgMatches) -> Vec<String> {
    sub_matches
        .get_many::<String>("ticket-id")
        .unwrap()
        .into_iter()
        .map(|v| v.to_string())
        .collect()
}

pub fn get_source_and_target_branches(
    sub_matches: &clap::ArgMatches,
    default_source: String,
) -> (String, String) {
    let input_source = sub_matches.get_one::<String>("source-branch");

    let mut source = default_source;
    let mut target = String::from("main");

    if let Some(s) = input_source {
        if !s.is_empty() {
            source = s.to_string();
        }
    }

    let input_target = sub_matches.get_one::<String>("target-branch");

    if let Some(t) = input_target {
        if !t.is_empty() {
            target = t.to_string();
        }
    }

    (source, target)
}
