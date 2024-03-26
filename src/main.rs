use std::path::Path;

use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs;

mod ai;
mod cli;
mod git;
mod providers;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    ai: ai::AIConfiguration,
    ticket: providers::TicketProviderConfiguration,
    instructions: Vec<String>,
}

impl Configuration {
    pub fn save(&self) {
        let configuration_file_path = Path::new("./git-message.json");
        let _ = fs::write(
            configuration_file_path,
            serde_json::to_string_pretty(&self).unwrap(),
        );
    }

    pub fn read() -> Configuration {
        let configuration_file_path = Path::new("./git-message.json");
        let file = fs::read_to_string(configuration_file_path).expect("Failed to read configuration");

        serde_json::from_str(&file).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli::direct::run().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let configuration = Configuration::read();

            let (diff, ticket_id) = tokio::join!(
                git::get_diff(),
                cli::get_ticket_id(&sub_matches, &configuration)
            );

            let get_ticket_result = providers::get_ticket(&configuration.ticket, ticket_id).await?;
            let ticket = if get_ticket_result.title.is_empty() {
                None
            } else {
                Some(get_ticket_result)
            };

            let message = ai::generate_message(configuration, ticket, diff.unwrap()).await?;

            println!("{:#?}", message);
        }
        Some(("initiate", _)) => {
            cli::interactive::initiate();

            println!("✅ Configuration saved with success!");
        },
        _ => panic!("command not found! Please try using --help"),
    }

    Ok(())
}
