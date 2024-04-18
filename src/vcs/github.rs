use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

use crate::git::Repository;
use crate::secrets::Variables;

pub async fn create_pr(pr_payload: super::CreatePRPayload) -> Result<Value, Error> {
    let github_token = Variables::GithubToken.get();
    let token = format!("Bearer {github_token}");
    
    let Repository { owner, name } = pr_payload.repository;
    let url = format!("https://api.github.com/repos/{owner}/{name}/pulls");

    let client = Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let json_body = serde_json::json!({
        "title": "",
        "body": "",
        "base": "",
        "head": ""
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
