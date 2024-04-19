use clap::ArgMatches;

pub mod direct;
pub mod interactive;

pub async fn get_ticket_ids(sub_matches: &ArgMatches, configuration: &super::Configuration) -> Vec<String> {
    if sub_matches.subcommand_matches("interactive").is_some() {
        interactive::get_task_ids(&configuration)
    } else {
        direct::get_task_ids(sub_matches)
    }
}

pub async fn get_source_and_target_branches(sub_matches: &ArgMatches, default_source: String) -> (String, String) {
    if sub_matches.subcommand_matches("interactive").is_some() {
        interactive::get_source_and_target_branches(default_source)
    } else {
        direct::get_source_and_target_branches(sub_matches, default_source)
    }
}