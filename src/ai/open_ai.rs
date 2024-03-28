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

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: String,
}

pub async fn generate_message(
    messages: serde_json::Value,
    configuration: &OpenAIConfiguration,
) -> Result<Value, Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let token = format!(
        "Bearer {}",
        env::var("GA_OPENAI_TOKEN").expect("Environment variable `GA_OPENAI_TOKEN`")
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
        "messages": messages,
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
    tickets: Vec<Ticket>,
    commit_instructions: Vec<String>,
) -> serde_json::Value {
    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system",
            content: String::from(
                "You should act as a senior software developer \
                that will generate beautiful git commit messages following the provided \
                instructions that would be easy to read. Whenever you receive a Ticket \
                information, you should use these information to help contextualize the \
                updates you will receive on git diff. You should follow the provided \
                instructions to create a message that matches with the user pattern. \
                You should take care of not violate the provided instructions. You \
                should not include any extra content on your response, only the final \
                commit message ready to be included on a git commit.",
            ),
        },
        ChatMessage {
            role: "user",
            content: format!(
                "This is the git diff:\n \
                ```\n\
                {}\n\
                ```",
                diff,
            ),
        },
    ];

    if commit_instructions.len() > 0 {
        messages.push(ChatMessage {
            role: "user",
            content: format!(
                "Follow the following instruction as much as possible:\n* {}",
                commit_instructions.join("* \n")
            ),
        })
    }

    if tickets.len() != 0 {
        messages.push(ChatMessage {
            role: "user",
            content: tickets
                .iter()
                .map(|ticket| {
                    format!(
                        "Task id: {}\nTask title: {}\nTask description: {}\n",
                        ticket.id, ticket.title, ticket.description,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n---\n"),
        });
    }

    serde_json::json!(messages)
}
