use std::env;

use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Error};

use futures::future;

use base64::engine::general_purpose::STANDARD;

use super::Ticket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraConfiguration {
    pub host: String,
    pub prefix: Option<String>,
}

pub fn get_client() -> (Client, HeaderMap) {
    let jira_user = env::var("GA_JIRA_USER").expect("Environment variable `JIRA_USER`");
    let jira_token = env::var("GA_JIRA_TOKEN").expect("Environment variable `JIRA_TOKEN`");

    let authorization = format!(
        "Basic {}",
        STANDARD.encode(format!("{}:{}", jira_user, jira_token)),
    );

    let client = Client::builder().cookie_store(true).build().unwrap();
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&authorization).unwrap(),
    );

    (client, headers)
}

pub async fn get_ticket(
    client: &Client,
    headers: &HeaderMap,
    host: &String,
    id: &String,
) -> Result<Value, Error> {
    let url = format!("https://{}.atlassian.net/rest/api/3/issue/{}", host, id);

    let response = client.get(url).headers(headers.clone()).send().await?;

    response.json::<Value>().await
}

pub async fn get_tickets(
    ticket_ids: Vec<String>,
    configuration: &JiraConfiguration,
) -> Result<Vec<Value>, Error> {
    let (client, headers) = get_client();

    let bodies = future::join_all(ticket_ids.iter().map(|id| {
        let client = &client;
        let headers = &headers;
        let host = &configuration.host;
        async move { get_ticket(client, headers, host, id).await }
    }))
    .await;

    let mut payloads = Vec::with_capacity(bodies.len());

    for response in bodies {
        match response {
            Ok(r) => payloads.push(r),
            Err(e) => {
                println!("ERROR: Getting JIRA Issue {}", e.to_string());
            }
        }
    }

    Ok(payloads)
}

pub fn get_fields(ticket: Value) -> Ticket {
    let mut fields = Ticket {
        id: String::new(),
        title: String::new(),
        description: String::new(),
    };

    if let Some(id) = ticket.get("key") {
        fields.id = id.as_str().unwrap().to_string();
    }

    if let Some(ticket_fields) = ticket.get("fields") {
        if let Some(summary) = ticket_fields.get("summary") {
            fields.title = summary.as_str().unwrap().to_string();
        }

        // TODO: Find a way to remove formatation
        // if let Some(description) = ticket_fields.get("description") {
        //     fields.description = description.as_str().unwrap().to_string();
        // }
    }

    fields
}
