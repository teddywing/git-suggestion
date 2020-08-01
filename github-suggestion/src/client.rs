use github_rs::client::{Executor, Github};
use serde_json::Value;
use thiserror::Error;

use crate::suggestion::Suggestion;


#[derive(Debug, Error)]
pub enum Error {
    #[error("GitHub client error: {0}")]
    Github(String),

    #[error("Unable to deserialize")]
    Deserialize(#[from] serde_json::error::Error),
}


pub struct Client<'a> {
    client: Github,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(
        token: &str,
        owner: &'a str, repo: &'a str,
    ) -> Result<Self, Error> {
        let client = match Github::new(&token) {
            Ok(g) => g,
            Err(e) => return Err(Error::Github(e.to_string())),
        };

        Ok(Client { client, owner, repo })
    }

    pub fn fetch(&self, id: &str) -> Result<Suggestion, Error> {
        let response = self.client
            .get()
            .repos()
            .owner(self.owner)
            .repo(self.repo)
            .pulls()
            .comments()
            .id(id)
            .execute::<Value>();

        match response {
            Ok((_, _, Some(json))) => {
                let suggestion = serde_json::from_value(json)?;

                Ok(suggestion)
            },
            Ok((_, _, None)) => Err(Error::Github("no response".to_owned())),
            Err(e) => Err(Error::Github(e.to_string())),
        }
    }
}
