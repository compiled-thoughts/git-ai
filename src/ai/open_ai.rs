use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

use crate::providers::Ticket;
use crate::secrets::Variables;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIConfiguration {
    pub model: String,
}

/// This represents a chat message with the openAI api.
///
/// This chat is composed by a conversation of the user,
/// this will be represented by this program talking as the user,
/// and the system that is the AI that will provide the response.
#[derive(Debug, Deserialize, Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: String,
}

/// Will generate a commit message based on current project git diff
/// output and, if enabled, the given ticket information.
pub async fn generate_message(
    messages: serde_json::Value,
    configuration: &OpenAIConfiguration,
) -> Result<Value, Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let token = format!("Bearer {}", Variables::OpenaiToken.get());

    let client = Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let json_body = serde_json::json!({
        "model": &configuration.model,
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
            println!("❌ ERROR: Generating the message {e}");
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
                {diff}\n\
                ```"
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

pub fn get_prompt_for_pull_request(
    commit_list: Vec<String>,
    ticket_list: Vec<Ticket>,
    pull_request_instructions: Option<Vec<String>>,
) -> serde_json::Value {
    let mut payload = vec![
        ChatMessage {
            role: "system",
            content: String::from(
                "You should act as a senior software developer \
                that will generate beautiful and clearly good pull request title and \
                body following the provided instructions and a list of commit message. \
                Whenever you receive a list of commits and maybe a task, you should \
                use these informations to help contextualize the reviewers in the pr \
                title and description which you will generate. You should follow the \
                instructions to do it. You should not include any extra content on your \
                response only a title at the first line and a body that can you more \
                than one line",
            ),
        },
        ChatMessage {
            role: "user",
            content: format!("List of commit messages:\n{}", commit_list.join("\n * ")),
        },
    ];

    if let Some(instructions) = pull_request_instructions {
        if instructions.len() != 0 {
            payload.push(ChatMessage {
                role: "user",
                content: format!(
                    "Follow this rule as strict as prossible:\n* {}",
                    instructions.join("\n * ")
                ),
            });
        }
    }

    if ticket_list.len() != 0 {
        let tickets = ticket_list
            .iter()
            .map(|ticket| {
                format!(
                    "Task title: {}\nTask description: {}\n",
                    ticket.title, ticket.description,
                )
            })
            .collect::<Vec<String>>()
            .join("\n---\n");

        payload.push(ChatMessage {
            role: "user",
            content: tickets,
        });
    }

    serde_json::json!(payload)
}
