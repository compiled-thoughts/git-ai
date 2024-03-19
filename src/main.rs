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

            let (diff, ticket_id) = tokio::join!(
                git::get_diff(),
                cli::get_ticket_id(&sub_matches, &configuration)
            );

            let ticket = providers::get_ticket(&configuration.ticket, ticket_id).await?;

            let message = ai::generate_message(configuration, ticket, diff.unwrap()).await?;

            // git::write_message(message);
            println!("{:#?}", message);
        }
        Some(("create", _)) => {
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
