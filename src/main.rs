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

            if *sub_matches.get_one("dry-run").unwrap() {
                message.split('\n').for_each(|m| println!("{}", m));
            } else {
            git::write_message(&message);
            }
        }
        Some(("create", sub_matches)) => {
            let configuration = Configuration::read();

            let ticket_ids: Vec<String> = cli::get_ticket_ids(sub_matches, &configuration).await;

            let (tickets, default_source_branch) = tokio::join!(
                providers::get_tickets(&configuration.ticket, ticket_ids),
                git::get_current_branch(),
            );

            let (head, base) =
                cli::get_source_and_target_branches(sub_matches, default_source_branch.unwrap())
                    .await;

            let commits = git::get_different_commits_between_branches(&head, &base).await;

            let message =
                ai::generate_pull_request(&configuration, tickets.unwrap(), commits.unwrap()).await?;

            let (title, body) =
                cli::interactive::get_message_change(&String::from(""), &String::from(""));

            let payload = vcs::PullRequestPayload {
                title,
                body,
                base,
                head,
            };

            vcs::create_pull_requet(configuration, payload).await?;

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
