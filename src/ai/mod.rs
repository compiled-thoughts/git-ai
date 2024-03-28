use serde::{Deserialize, Serialize};

pub mod open_ai;

use crate::providers::Ticket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AIConfiguration {
    OpenAI(open_ai::OpenAIConfiguration),
}

pub const AVAILABLE_AI: &[&str; 1] = &["OpenAI"];

pub async fn generate_message(
    configuration: super::Configuration,
    ticket: Vec<Ticket>,
    diff: String,
) -> Result<String, reqwest::Error> {
    match &configuration.ai {
        AIConfiguration::OpenAI(open_ai_configuration) => {
            let commit_payload =
                open_ai::get_prompt_for_commit(diff, ticket, configuration.instructions.commit);

            let payload = open_ai::generate_message(commit_payload, open_ai_configuration).await?;

            let message = &payload["choices"][0]["message"]["content"];

            Ok(message.to_string())
        } // _ => { unreachable!("AI not found!") },
    }
}
) -> Result<serde_json::Value, reqwest::Error> {
    match &configuration.ai {
        AIConfiguration::OpenAI(open_ai_configuration) => {
            let commit_payload =
                open_ai::get_prompt_for_commit(diff, ticket, configuration.commit_instructions);

            let message = open_ai::generate_message(commit_payload, open_ai_configuration).await?;

            Ok(message)
        } // _ => { unreachable!("AI not found!") },
    }
}
