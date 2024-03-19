use serde::{Deserialize, Serialize};
use std::{fs, iter::FromIterator, path::Path};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

fn remove_unecessary_lines(diff: &String) -> String {
    diff.split("\n")
        .filter(|l| l.starts_with("index"))
        .collect()
}

fn get_repository_name(remote: &Vec<&str>) -> Result<String, std::io::Error> {
    let repository_name = remote.last();

    if let Some(name) = repository_name {
        if !name.is_empty() {
            return Ok(name.replace(".git", ""));
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Not found repository name",
    ))
}

fn get_repository_owner(remote: &Vec<&str>) -> Result<String, std::io::Error> {
    let repository_owner = remote.get(remote.len() - 2);

    if let Some(owner) = repository_owner {
        if !owner.is_empty() {
            return Ok(owner.to_string());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Not found repository owner",
    ))
}

pub async fn get_remote_origin() -> Result<Repository, std::io::Error> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .await?;

    let remote = String::from_iter(output.stdout.iter().map(|&c| c as char));

    if !remote.is_empty() {
        let remote_splited = remote.split(|c| c == '/' || c == ':').collect();
        let name = get_repository_name(&remote_splited)?;
        let owner = get_repository_owner(&remote_splited)?;

        Ok(Repository { name, owner })
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Git remote not found",
        ))
    }
}

pub async fn get_diff() -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["status", "-v"]).output().await?;

    let diff = String::from_iter(output.stdout.iter().map(|&c| c as char));

    if !diff.is_empty() {
        Ok(remove_unecessary_lines(&diff))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Git diff not found",
        ))
    }
}

pub fn write_message(message: String) {
    let git_message_file = Path::new("./.git/COMMIT_EDITMSG");
    let current_message =
        fs::read_to_string(git_message_file).expect("Failed to read actual message");

    let mut lines: Vec<String> = current_message.split("\n").map(|s| s.to_string()).collect();

    lines[0] = message;

    let _ = fs::write(git_message_file, lines.concat());
}
