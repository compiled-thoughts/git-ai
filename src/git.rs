use std::iter::FromIterator;
use tokio::process::Command;
pub async fn get_diff() -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .await?;

    let diff = String::from_iter(output.stdout.iter().map(|&c| c as char));

    if !diff.is_empty() {
        Ok(diff)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Git diff not found",
        ))
    }
}
