// Copyright (c) 2020  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use github_rs::client::{Executor, Github};
use serde_json::Value;
use thiserror::Error;

use crate::suggestion::Suggestion;


/// Client and network errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("GitHub client error: {0}")]
    Github(String),

    #[error("Unable to deserialize")]
    Deserialize(#[from] serde_json::error::Error),
}


/// A GitHub client wrapper for a specific repository.
pub struct Client<'a> {
    client: Github,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> Client<'a> {
    /// Create a new GitHub client.
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

    /// Fetch a suggestion comment from GitHub by its ID.
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
