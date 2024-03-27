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

            let get_ticket_result = providers::get_ticket(&configuration.ticket, ticket_id).await?;
            let ticket = if get_ticket_result.title.is_empty() {
                None
            } else {
                Some(get_ticket_result)
            };

            let message = ai::generate_message(configuration, ticket, diff.unwrap()).await?;

            let content = message.get("choices").unwrap()[0]
                .get("message")
                .unwrap()
                .get("content")
                .unwrap()
                .to_string()
                .replace("\"", "")
                .replace("\\n", "\n");

            println!("{}", content);
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
