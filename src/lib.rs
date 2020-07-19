#![warn(rust_2018_idioms)]


use github_rs::client::{Executor, Github};
use serde_json::Value;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("GitHub client error: {0}")]
    Github(String),
}


pub struct Suggestion<'a> {
    client: Github,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> Suggestion<'a> {
    pub fn new(token: &str, owner: &'a str, repo: &'a str) -> Self {
        let client = Github::new(&token).unwrap();

        Suggestion { client, owner, repo }
    }

    pub fn fetch(&self, id: &str) -> Result<(), Error> {
        let comment = self.client
            .get()
            .repos()
            .owner(self.owner)
            .repo(self.repo)
            .pulls()
            .comments()
            .id(id)
            .execute::<Value>();

        match comment {
            Ok((_, _, json)) => {
                println!("{:?}", json);

                Ok(())
            },
            Err(e) => Err(Error::Github(e.to_string())),
        }
    }

    pub fn patch(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_fetch_gets_pull_request_comment() {
        let suggestion = Suggestion::new(
            env!("GITHUB_TOKEN"),
            "cli",
            "cli",
        );
        suggestion.fetch("438947607").unwrap();
    }
}
