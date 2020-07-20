#![warn(rust_2018_idioms)]


use github_rs::client::{Executor, Github};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;


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
    pub fn new(token: &str, owner: &'a str, repo: &'a str) -> Self {
        let client = Github::new(&token).unwrap();

        Client { client, owner, repo }
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

#[derive(Debug, Deserialize)]
pub struct Suggestion {
    #[serde(rename = "diff_hunk")]
    diff: String,

    #[serde(rename = "body")]
    suggestion: String,
}

impl Suggestion {
    pub fn patch(&self) -> String {
        let mut diff: Vec<_> = self.diff.lines()
            .filter(|l| !l.starts_with("-"))
            .map(|l| {
                if l.starts_with("+") {
                    return l.replacen("+", " ", 1);
                }

                l.to_owned()
            })
            .collect();

        let last = diff.len() - 1;
        diff[last] = diff.last().unwrap()
            .replacen(" ", "-", 1);

        diff.push(self.suggestion());

        diff.join("\n")
    }

    fn suggestion(&self) -> String {
        let re = Regex::new(r"(?s).*```\s*suggestion\n").unwrap();
        let s = re.replace(&self.suggestion, "+");
        s.replace("```", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_fetch_gets_pull_request_comment() {
        let client = Client::new(
            env!("GITHUB_TOKEN"),
            "cli",
            "cli",
        );

        let suggestion = client.fetch("438947607").unwrap();

        println!("{:?}", suggestion);
    }

    #[test]
    fn suggestion_patch_generates_patch() {
        let suggestion = Suggestion {
            diff: r#"@@ -1, 9 +1, 11 @@
 package command
 
 import (
+	"bufio" // used to input comment
 	"errors"
 	"fmt"
 	"io"
+	"os" // used to input comment"#.to_owned(),
            suggestion: r#"It's ok to leave these uncommented

```suggestion
	"os"
```"#.to_owned(),
        };

        assert_eq!(
            suggestion.patch(),
            r#"@@ -1, 9 +1, 11 @@
 package command
 
 import (
 	"bufio" // used to input comment
 	"errors"
 	"fmt"
 	"io"
-	"os" // used to input comment
+	"os"
"#,
        );
    }
}
