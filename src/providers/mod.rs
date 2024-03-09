use reqwest::Error;
use serde::{Deserialize, Serialize};

pub mod jira;

#[derive(Debug)]
pub struct Ticket {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketProviderConfiguration {
    JIRA(jira::JiraConfiguration),
}

pub const AVAILABLE_TICKET_PROVIDERS: &[&str; 3] = &["Github", "JIRA", "Trello"];

pub async fn get_ticket(
    configuration: &TicketProviderConfiguration,
    ticket_id: String,
) -> Result<Ticket, Error> {
    match &configuration {
        TicketProviderConfiguration::JIRA(jira) => {
            let payload = jira::get_ticket(ticket_id, &jira).await?;
            Ok(jira::get_fields(payload))
        } // _ => { unreachable!("Ticket provider not found!"); },
    }
}
