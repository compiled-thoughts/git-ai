use serde::{Deserialize, Serialize};

pub mod open_ai;

use crate::providers::Ticket;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AIConfiguration {
    OpenAI(open_ai::OpenAIConfiguration),
}

/// A list of available AI API integrations
pub const AVAILABLE_AI: &[&str; 1] = &["OpenAI"];

/// This method will choose the defined AI
/// on the configurations to run the
/// commit message.
///
/// TODO: for now, we have only the openAI implementation
pub async fn generate_message(
    configuration: super::Configuration,
    tickets: Vec<Ticket>,
    diff: String,
) -> Result<String, reqwest::Error> {
    match &configuration.ai {
        AIConfiguration::OpenAI(open_ai_configuration) => {
            let commit_payload =
                open_ai::get_prompt_for_commit(
                    diff, 
                    tickets, 
                    configuration.instructions.commit
                );

            let payload = open_ai::generate_message(
                commit_payload, 
                open_ai_configuration
            ).await?;

            let message = &payload["choices"][0]["message"]["content"];

            Ok(message.to_string())
        } // _ => { unreachable!("AI not found!") },
    }
}

pub async fn generate_pull_request(
    configuration: &super::Configuration,
    tickets: Vec<Ticket>,
    commits: Vec<String>,
) -> Result<serde_json::Value, reqwest::Error> {
    match &configuration.ai {
        AIConfiguration::OpenAI(open_ai_configuration) => {
            let commit_payload = open_ai::get_prompt_for_pull_request(
                commits,
                tickets,
                configuration.instructions.pr.clone(),
            );

            let message = open_ai::generate_message(commit_payload, open_ai_configuration).await?;

            Ok(message)
        } // _ => { unreachable!("AI not found!") },
    }
}
