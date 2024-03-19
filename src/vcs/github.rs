use std::env;

use serde_json::Value;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};


pub async fn create_pr(pr_payload: super::CreatePRPayload) -> Result<Value, Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        pr_payload.repository.owner, pr_payload.repository.name
    );
    let token = format!(
        "Bearer {}",
        env::var("GA_GITHUB_TOKEN").expect("Environment variable `GA_GITHUB_TOKEN`")
    );

    let client = Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let json_body = serde_json::json!({
        "title": "",
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
