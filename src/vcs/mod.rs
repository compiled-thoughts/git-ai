use serde::{Deserialize, Serialize};

use crate::git;

pub mod github;

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestPayload {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePRPayload {
    pub content: PullRequestPayload,
    pub repository: git::Repository,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VCSConfiguration {
    Github,
}

impl VCSConfiguration {
    const GITHUB_STR: &'static str = "Github";

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(VCSConfiguration::Github),
            _ => None,
        }
    }
}

pub static AVAILABLE_VCS: &[&str; 1] = &[VCSConfiguration::GITHUB_STR];

pub async fn create_pull_requet(
    configuration: super::Configuration,
    content: PullRequestPayload,
) -> Result<(), reqwest::Error> {
    let repository = git::get_remote_origin()
        .await
        .expect("Can't get repository name and owner");

    match &configuration.vcs {
        Some(VCSConfiguration::Github) => {
            github::create_pr(CreatePRPayload {
                repository,
                content,
            });

            Ok(())
        }
        _ => unreachable!("Invalid `vcs` configuration!"),
    }
}
