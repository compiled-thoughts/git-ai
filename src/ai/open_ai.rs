use std::env;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

use crate::providers::Ticket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIConfiguration {
    pub model: String,
}

pub async fn generate_message(
    prompt: serde_json::Value,
    configuration: &OpenAIConfiguration,
) -> Result<Value, Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let token = format!(
        "Bearer {}",
        env::var("GA_OPENAI_TOKEN").expect("Environment variable `OPENAI_TOKEN`")
    );

    let model = &configuration.model;

    let client = Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let json_body = serde_json::json!({
        "model": model,
        "temperature": 0.2,
        "messages": prompt,
    });

    let response = client
        .post(url)
        .headers(headers)
        .body(serde_json::to_string(&json_body).unwrap())
        .send()
        .await;

    match response {
        Ok(r) => r.json::<Value>().await,
        Err(e) => {
            println!("ERROR: Generating the message {}", e.to_string());
            Ok(Value::Null)
        }
    }
}

pub fn get_prompt_for_commit(
    diff: String,
    ticket: Ticket,
    commit_instructions: Vec<String>,
) -> serde_json::Value {
    serde_json::json!([
        {
            "role": "system",
            "content": "You are a very git message generator which receives the user input and output a great message following the patterns provided by the user input considering these contexts the diff of the commits and the related task or ticket."
        },
        {
            "role": "user",
            "content": format!(
                "Follow this pattern as strict as prossible:\n* {}",
                commit_instructions.join("\n * ")
            )
        },
        {
            "role": "user",
            "content": format!(
                "Task title: {}\nTask description: {}\n",
                ticket.title,
                ticket.description,
            )
        },
        {
            "role": "user",
            "content": format!(
                "Git diff: {}",
                diff,
            )
        }
    ])
}
