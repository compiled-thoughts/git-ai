use std::path::Path;

use serde::{Deserialize, Serialize};
use std::fs;

use crate::vcs;
use crate::{ai, providers};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub ai: ai::AIConfiguration,
    pub ticket: providers::TicketProviderConfiguration,
    pub commit_instructions: Vec<String>,
    
    pub vcs: Option<vcs::VCSConfiguration>,
    pub pull_request_instructions: Option<Vec<String>>,
}

static CONFIGURATION_FILE: &str = "./git-message.json";

impl Configuration {
    pub fn save(&self) {
        let configuration_file_path = Path::new(CONFIGURATION_FILE);
        let _ = fs::write(
            configuration_file_path,
            serde_json::to_string_pretty(&self).unwrap(),
        );
    }

    pub fn read() -> Configuration {
        let configuration_file_path = Path::new(CONFIGURATION_FILE);
        let file = fs::read_to_string(configuration_file_path)
            .expect(format!("Failed to read configuration file `{}`", CONFIGURATION_FILE).as_str());

        serde_json::from_str(&file).unwrap()
    }
}
