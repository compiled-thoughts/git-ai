use serde::{Deserialize, Serialize};

pub mod open_ai;

use crate::providers::Ticket;

pub struct MessagePayload {
    pub diff: String,
    pub ticket: Option<Ticket>,
    pub instructions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AIConfiguration {
    OpenAI(open_ai::OpenAIConfiguration),
}

pub const AVAILABLE_AI: &[&str; 1] = &["OpenAI"];

pub async fn generate_message(
    configuration: super::Configuration,
    ticket: Option<Ticket>,
    diff: String,
) -> Result<serde_json::Value, reqwest::Error> {
    match &configuration.ai {
        AIConfiguration::OpenAI(open_ai_configuration) => {
            let message = open_ai::generate_message(
                MessagePayload {
                    ticket,
                    diff,
                    instructions: configuration.instructions.clone(),
                },
                open_ai_configuration,
            )
            .await?;

            Ok(message)
        }
        // _ => { unreachable!("AI not found!") },
    }
}
