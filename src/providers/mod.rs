use reqwest::Error;
use serde::{Deserialize, Serialize};

pub mod jira;

#[derive(Debug)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketProviderConfiguration {
    JIRA(jira::JiraConfiguration),
}

pub const AVAILABLE_TICKET_PROVIDERS: &[&str; 4] = &["Github", "JIRA", "Trello", "None"];

pub async fn get_tickets(
    configuration: &Option<TicketProviderConfiguration>,
    ticket_ids: Vec<String>,
) -> Result<Vec<Ticket>, Error> {
    if ticket_ids.len() == 0 || configuration.is_none() {
        return Ok(Vec::new());
    }

    match configuration.as_ref().unwrap() {
        TicketProviderConfiguration::JIRA(jira) => {
            let payloads = jira::get_tickets(ticket_ids, &jira).await?;

            let tickets = payloads
                .into_iter()
                .map(|payload| jira::get_fields(payload))
                .collect::<Vec<Ticket>>();

            Ok(tickets)
        }
    }
}
