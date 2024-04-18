use std::env;

use convert_case::{Case, Casing};

static PREFIX: &str = "GA";

#[derive(strum_macros::Display)]
pub enum Variables {
    OpenaiToken,

    JiraUser,
    JiraToken,

    GithubToken,
}

impl Variables {
    pub fn get(&self) -> String {
        let key = format!("{PREFIX}{self}").to_case(Case::ScreamingSnake);
        env::var(&key).expect(&format!("Environment variable `{key}` is not defined"))
    }
}
