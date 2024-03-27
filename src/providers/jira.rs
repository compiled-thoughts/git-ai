use std::env;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Error};

use super::Ticket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraConfiguration {
    pub host: String,
    pub prefix: Option<String>,
}

pub async fn get_ticket(
    id: String,
    configuration: &JiraConfiguration,
) -> Result<Option<Value>, Error> {
    if id.is_empty() {
        return Ok(None);
    }
    let url = format!(
        "https://{}.atlassian.net/rest/api/2/issue/{}",
        configuration.host, id
    );

    let authorization = env::var("GMA_JIRA_AUTHORIZATION").expect("Environment `JIRA_TOKEN`");

    let client = Client::builder().cookie_store(true).build().unwrap();
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&authorization).unwrap(),
    );

    let response = client.get(url).headers(headers).send().await;

    match response {
        Ok(r) => Ok(Some((r.json::<Value>().await).unwrap())),
        Err(e) => {
            println!("ERROR: Getting JIRA Issue {}", e.to_string());
            Ok(Some(Value::Null))
        }
    }
}

pub fn get_fields(ticket: Option<Value>) -> Ticket {
    let mut fields = Ticket {
        title: String::new(),
        description: String::new(),
    };

    match ticket {
        Some(value) => {
            if let Some(ticket_fields) = value.get("fields") {
                if let Some(description) = ticket_fields.get("description") {
                    fields.description = description.as_str().unwrap().to_string();
                }

                if let Some(summary) = ticket_fields.get("summary") {
                    fields.title = summary.as_str().unwrap().to_string();
                }
            }
        },
        None => {}
    }

    fields
}
