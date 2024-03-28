use std::path::Path;

use serde::{Deserialize, Serialize};
use std::fs;

use crate::vcs;
use crate::{ai, providers};

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    pub commit: Vec<String>,
    pub pr: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub ai: ai::AIConfiguration,
    pub instructions: Instructions,

    pub vcs: Option<vcs::VCSConfiguration>,
    pub ticket: Option<providers::TicketProviderConfiguration>,
}

static CONFIGURATION_FILE: &str = "./git-message.json";

impl Configuration {
    pub fn new() -> Self {
        let open_ai_configuration = ai::open_ai::OpenAIConfiguration {
            model: String::from("gpt-3.5-turbo"),
        };
        let default_ai = ai::AIConfiguration::OpenAI(open_ai_configuration);

        Configuration {
            instructions: Instructions {
                commit: Vec::new(),
                pr: None,
            },
            ai: default_ai,

            ticket: None,
            vcs: None,
        }
    }

    pub fn save(&self) {
        const CONFIGURATION_FILE_PATH: &Path = Path::new(CONFIGURATION_FILE);
        
        let _ = fs::write(
            CONFIGURATION_FILE_PATH,
            serde_json::to_string_pretty(&self).unwrap(),
        );
    }

    pub fn read() -> Self {
        const CONFIGURATION_FILE_PATH: &Path = Path::new(CONFIGURATION_FILE);

        if !CONFIGURATION_FILE_PATH.exists() {
            return Self::new();
        }

        let file = fs::read_to_string(CONFIGURATION_FILE_PATH)
            .expect(format!("Failed to read configuration file `{}`", CONFIGURATION_FILE).as_str());

        serde_json::from_str(&file).unwrap()
    }
}
