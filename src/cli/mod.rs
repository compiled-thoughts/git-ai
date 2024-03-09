use clap::ArgMatches;

pub mod direct;
pub mod interactive;

pub async fn get_ticket_id(sub_matches: &ArgMatches, configuration: &super::Configuration) -> String {
    if sub_matches.subcommand_matches("interactive").is_some() {
        interactive::generate(&configuration)
    } else {
        direct::generate(sub_matches, &configuration)
    }
}
