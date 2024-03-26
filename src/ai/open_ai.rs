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
                r#"You are a git message generator which receives the user input and output a great message
        following the patterns provided by the user input considering these contexts:
  the git diff of the commits and the related task or ticket.
  "#,
            ),
        },
        ChatMessage {
            role: "user".to_string(),
            content: format!(
                "Follow this pattern as strict as prossible:\n* {}",
                payload.instructions.join("\n * ")
            ),
        },
        ChatMessage {
            role: "user".to_string(),
            content: format!(
                r#"
        Based on the following git commit diff, generate a beautiful and compreensive
        git commit message following the Semantic Commit format and using emojis whenever you need.
        You should not include any content on your response, only the commit contet ready to be used.

```
{}
```

        The commit message should pass in the Semantic Commit format validation.
            \n\n"#,
                payload.diff,
            ),
        },
    ];
    match &payload.ticket {
        Some(ticket) => messages.push(ChatMessage {
            role: "user".to_string(),
            content: format!(
                "Task title: {}\nTask description: {}\n",
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
