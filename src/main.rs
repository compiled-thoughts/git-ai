use reqwest::Error;

mod ai;
mod cli;
mod configuration;
mod git;
mod providers;
mod vcs;

use configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli::direct::run().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let configuration = Configuration::read();

            let (diff, ticket_ids) = tokio::join!(
                git::get_diff(),
                cli::get_ticket_ids(&sub_matches, &configuration)
            );

            let tickets = providers::get_tickets(&configuration.ticket, ticket_ids).await?;

            let message = ai::generate_message(configuration, tickets, diff.unwrap()).await?;

            println!("{}", &message);
            
            git::write_message(&message);
        }
            let configuration = Configuration::read();

            let payload = vcs::PullRequestPayload {
                title: String::from(""),
                body: String::from(""),
            };

            vcs::create_pull_requet(configuration, payload);

            println!("✅ Pull request created with success!");
        }
        Some(("initiate", _)) => {
            cli::interactive::initiate();

            println!("✅ Configuration saved with success!");
        }
        _ => panic!("❌ Command not found! Please try using --help"),
    }

    Ok(())
}
