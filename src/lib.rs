#![warn(rust_2018_idioms)]


mod url;

pub use crate::url::SuggestionUrl;


use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use git2::Repository;
use github_rs::client::{Executor, Github};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use tempfile::NamedTempFile;


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
    comment: String,

    path: String,

    original_start_line: Option<usize>,

    #[serde(rename = "original_line")]
    original_end_line: usize,
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

        diff.push(self.suggestion_patch());

        diff.join("\n")
    }

    fn suggestion_patch(&self) -> String {
        let re = Regex::new(r"(?s).*(?-s)```\s*suggestion.*\n").unwrap();
        let s = re.replace(&self.comment, "+");
        s.replace("```", "")
    }

    fn suggestion(&self) -> String {
        let re = Regex::new(r"(?s).*(?-s)```\s*suggestion.*\n").unwrap();
        let s = re.replace(&self.comment, "");
        s.replace("```", "")
    }

    pub fn apply(&self) -> Result<(), Error> {
        let repo = Repository::open(".").unwrap();
        let repo_root = repo.workdir().unwrap();

        // let original = File::open(repo_root.join(&self.path)).unwrap();
        // let metadata = original.metadata().unwrap();
        // let created_at = metadata.created().unwrap();

        let new = NamedTempFile::new().unwrap();

        fs::copy(repo_root.join(&self.path), new.path()).unwrap();

        let mut original = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(repo_root.join(&self.path)).unwrap();

        self.apply_to(new.path(), &mut original)
    }

    fn apply_to<P: AsRef<Path>, W: Write>(
        &self,
        from: P,
        writer: &mut W,
    ) -> Result<(), Error> {
        let original = File::open(from).unwrap();
        let reader = BufReader::new(original);

        for (i, line) in reader.lines().enumerate() {
            let line_number = i + 1;

            match line {
                Ok(l) => {
                    if line_number == self.original_end_line {
                        write!(writer, "{}", self.suggestion()).unwrap();
                    } else if self.original_start_line.is_none()
                            || line_number < self.original_start_line.unwrap()
                            || line_number > self.original_end_line {
                        writeln!(writer, "{}", l).unwrap();
                    }
                },
                Err(e) => panic!(e),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
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
        // Diff from gabgodBB (https://github.com/gabgodBB) and suggestion from
        // probablycorey (https://github.com/probablycorey) in this pull
        // request: https://github.com/cli/cli/pull/1123

        let suggestion = Suggestion {
            diff: r#"@@ -1, 9 +1, 11 @@
 package command
 
 import (
+	"bufio" // used to input comment
 	"errors"
 	"fmt"
 	"io"
+	"os" // used to input comment"#.to_owned(),
            comment: r#"It's ok to leave these uncommented

```suggestion
	"os"
```"#.to_owned(),
            path: "".to_owned(),
            original_start_line: Some(8),
            original_end_line: 8,
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

    #[test]
    fn unified_diff() {
        use unidiff::PatchSet;

        let diff = r#"--- a/command/pr.go
+++ b/command/pr.go
@@ -1,9 +1,11 @@
 package command
 
 import (
+	"bufio" // used to input comment
 	"errors"
 	"fmt"
 	"io"
+	"os" // used to input comment
"#;

        let mut patch = PatchSet::new();
        patch.parse(diff).unwrap();

        println!("{:?}", patch);
        println!("{}", patch);

        let lines = patch.files_mut()[0].hunks_mut()[0].lines_mut();

        // for line in &lines {
        //     if line.is_removed() {
        //     } else if line.is_added() {
        //         line.line_type = unidiff::LINE_TYPE_CONTEXT.to_owned();
        //     }
        // }

        lines
            .iter_mut()
            .filter(|l| !l.is_removed())
            // .map(|l| {
            .for_each(|l| {
                if l.is_added() {
                    l.line_type = unidiff::LINE_TYPE_CONTEXT.to_owned();
                }
            });

        lines[lines.len() - 2].line_type = unidiff::LINE_TYPE_REMOVED.to_owned();

        patch.files_mut()[0].hunks_mut()[0].append(unidiff::Line::new(
            r#"	"os""#,
            unidiff::LINE_TYPE_ADDED,
        ));

        println!("{}", patch);
    }

    #[test]
    fn patches_file() {
        // File::open("../testdata/");
    }

    #[test]
    fn read_git_blob() {
        use std::path::Path;

        use git2::Repository;

        let repo = Repository::open("./private/suggestion-test").unwrap();
        let commit = repo.find_commit("b58be52880a0a0c0d397052351be31f19acdeca4".parse().unwrap()).unwrap();

        let object = commit
            .tree().unwrap()
            .get_path(Path::new("src/server.rs")).unwrap()
            .to_object(&repo).unwrap();

        let blob = object
            .as_blob().unwrap()
            .content();

        println!("{:?}", commit);
        println!("{}", std::str::from_utf8(blob).unwrap());
    }

    #[test]
    fn suggestion_apply_to_writes_patch_to_writer() {
        use std::io::Cursor;

        use tempfile::NamedTempFile;


        let mut temp = NamedTempFile::new().unwrap();
        let original = r#"
     ‘Beware the Jabberwock, my son!
      The jaws that bite, the claws that catch!
     Beware the Jubjub bird, and shun
      The frumious Bandersnatch!’

     He took his vorpal blade in hand:
      Long time the manxome foe he sought--
     So rested he by the Tumtum tree,
      And stood awhile in thought.
"#;

        write!(temp, "{}", original).unwrap();

        let suggestion = Suggestion {
            diff: "".to_owned(),
            comment: r#"``` suggestion
     He took his vorpal sword in hand:
      Long time the manxome foe he sought—
```"#.to_owned(),
            path: temp.path().to_string_lossy().to_string(),
            original_start_line: Some(6),
            original_end_line: 7,
        };

        let expected = r#"
     ‘Beware the Jabberwock, my son!
      The jaws that bite, the claws that catch!
     Beware the Jubjub bird, and shun
      The frumious Bandersnatch!’

     He took his vorpal sword in hand:
      Long time the manxome foe he sought—
     So rested he by the Tumtum tree,
      And stood awhile in thought.
"#;

        let mut actual = Cursor::new(Vec::new());
        suggestion.apply_to(temp.path(), &mut actual).unwrap();

        assert_eq!(
            std::str::from_utf8(&actual.into_inner()).unwrap(),
            expected,
        );
    }
}
