use std::env;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

use super::MessagePayload;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIConfiguration {
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

pub async fn generate_message(
    payload: MessagePayload,
    configuration: &OpenAIConfiguration,
) -> Result<Value, Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let token = format!(
        "Bearer {}",
        env::var("GMA_OPENAI_TOKEN").expect("Environment `GMA_OPENAI_TOKEN`")
    );

    let model = &configuration.model;

    let client = Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: String::from("system"),
            content: String::from(
                r#"You should act as a senios software developer that will generate git commit messages following
the semantic commit convention to generate beautiful commit message that if easy to read.
Wenever you receive a Ticket information, you should use these information to help contextualize the
updates you will receive on git diff.
You should use emojis whenever it is usefull to help the developers understand what was implemented in the git diff.
You should take care of not violate the semantic commit rules when use emojis.
You should not include any extra content on your response, only the final commit message ready to be included on a git commit.
  "#,
            ),
        },
        ChatMessage {
            role: "user".to_string(),
            content: format!(
                r#"
This is the git diff:
```
{}
```
            \n\n"#,
                payload.diff,
            ),
        },
    ];
    if payload.instructions.len() > 0 {
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: format!(
                "Follow the following instruction as much as possible:\n* {}",
                payload.instructions.join("* \n")
            ),
        })
    }
    match &payload.ticket {
        Some(ticket) => messages.push(ChatMessage {
            role: "user".to_string(),
            content: format!(
                "Ticket title: {}\nTicket description: {}\n",
                ticket.title, ticket.description,
            ),
        }),
        None => {}
    }
    let json_body = serde_json::json!({
        "model": model,
        "temperature": 0.2,
        "messages": messages
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
