use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use git2::{Patch, Repository};
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Git(#[from] git2::Error),

    #[error("{0} is not a blob")]
    GitObjectNotBlob(git2::Oid),

    #[error("{message}")]
    BufWriter {
        source: std::io::IntoInnerError<BufWriter<Vec<u8>>>,
        message: String,
    },

    #[error("{message}: {source}")]
    Io {
        source: std::io::Error,
        message: String,
    },

    #[error("{0} is not valid UTF-8")]
    InvalidUtf8(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

#[derive(Debug, PartialEq)]
enum LineEnding {
    Lf,
    CrLf,
}

#[derive(Debug, Deserialize)]
pub struct Suggestion {
    #[serde(rename = "diff_hunk")]
    diff: String,

    #[serde(rename = "body")]
    comment: String,

    #[serde(rename = "original_commit_id")]
    commit: String,

    path: String,

    original_start_line: Option<usize>,

    #[serde(rename = "original_line")]
    original_end_line: usize,
}

impl Suggestion {
    pub fn diff(&self) -> Result<String, Error> {
        let repo = Repository::open(".")?;

        self.diff_with_repo(&repo)
    }

    fn diff_with_repo(&self, repo: &Repository) -> Result<String, Error> {
        let commit = repo.find_commit(self.commit.parse()?)?;

        let path = Path::new(&self.path);

        let object = commit
            .tree()?
            .get_path(path)?
            .to_object(&repo)?;

        let blob = object.as_blob()
            .ok_or_else(|| Error::GitObjectNotBlob(object.id()))?;

        let blob_reader = BufReader::new(blob.content());
        let mut new = BufWriter::new(Vec::new());
        self.apply_to(blob_reader, &mut new)?;
        let new_buffer = new.into_inner()
            .map_err(|e| Error::BufWriter {
                source: e,
                message: "unable to read right side of patch".to_owned(),
            })?;

        let mut diff = Patch::from_blob_and_buffer(
            blob,
            Some(&path),
            &new_buffer,
            Some(&path),
            None,
        )?;

        Ok(
            diff.to_buf()?
                .as_str()
                .ok_or_else(|| Error::InvalidUtf8("diff".to_owned()))?
                .to_owned()
        )
    }

    fn suggestion_with_line_ending(
        &self,
        line_ending: &LineEnding,
    ) -> Result<String, Error> {
        let re = Regex::new(r"(?s).*(?-s)```\s*suggestion.*\n")?;
        let s = re.replace(&self.comment, "");
        let s = s.replace("```", "");

        // Suggestion blocks use CRLF by default.
        if *line_ending == LineEnding::Lf {
            return Ok(s.replace('\r', ""));
        }

        Ok(s)
    }

    pub fn apply(&self) -> Result<(), Error> {
        let repo = Repository::open(".")?;

        let diff_text = self.diff_with_repo(&repo)?;
        let diff = git2::Diff::from_buffer(diff_text.as_bytes())?;

        repo.apply(
            &diff,
            git2::ApplyLocation::WorkDir,
            None,
        )?;

        Ok(())
    }

    fn apply_to<R: BufRead, W: Write>(
        &self,
        reader: R,
        writer: &mut W,
    ) -> Result<(), Error> {
        let mut line_ending = LineEnding::Lf;

        for (i, line) in reader.lines().enumerate() {
            let line_number = i + 1;

            let line = line.map_err(|e| Error::Io {
                source: e,
                message: "Unable to read line".to_owned(),
            })?;

            // Determine which line endings the file uses by looking at the
            // first line.
            if line_number == 1 && is_line_crlf(&line) {
                line_ending = LineEnding::CrLf;
            }

            if line_number == self.original_end_line {
                write!(
                    writer,
                    "{}",
                    self.suggestion_with_line_ending(&line_ending)?,
                ).map_err(|e| Error::Io {
                    source: e,
                    message: "Write error".to_owned(),
                })?;
            } else if self.original_start_line.is_none()
                    || line_number < self.original_start_line.unwrap()
                    || line_number > self.original_end_line {
                writeln!(writer, "{}", line)
                    .map_err(|e| Error::Io {
                        source: e,
                        message: "Write error".to_owned(),
                    })?;
            }
        }

        Ok(())
    }
}

/// Determine the line ending for `line`.
///
/// If the second-to-last character on the first line is "\r", assume CRLF.
/// Otherwise, default to LF.
fn is_line_crlf(line: &str) -> bool {
    if let Some(c) = line.chars().rev().nth(2) {
        if c == '\r' {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_diff_with_repo_generates_diff() {
        use tempfile::tempdir;


        let git_root = tempdir().unwrap();
        let repo = Repository::init(git_root.path()).unwrap();

        let file = r#"
     ‘Beware the Jabberwock, my son!
      The jaws that bite, the claws that catch!
     Beware the Jubjub bird, and shun
      The frumious Bandersnatch!’

     He took his vorpal blade in hand:
      Long time the manxome foe he sought--
     So rested he by the Tumtum tree,
      And stood awhile in thought.
"#;

        let path = "poems/Jabberwocky.txt";

        let mut index = repo.index().unwrap();
        index.add_frombuffer(
            &git2::IndexEntry {
                ctime: git2::IndexTime::new(0, 0),
                mtime: git2::IndexTime::new(0, 0),
                dev: 0,
                ino: 0,
                mode: 0o100644,
                uid: 0,
                gid: 0,
                file_size: file.len() as u32,
                id: git2::Oid::zero(),
                flags: 0,
                flags_extended: 0,
                path: path.as_bytes().to_vec(),
            },
            file.as_bytes(),
        ).unwrap();
        let tree_oid = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();

        let author = git2::Signature::now(
            "Oshino Shinobu",
            "oshino.shinobu@example.com",
        ).unwrap();

        let commit = repo.commit(
            Some("HEAD"),
            &author,
            &author,
            "Sample commit",
            &tree,
            &[],
        ).unwrap();

        let suggestion = Suggestion {
            diff: "".to_owned(),
            comment: r#"``` suggestion
     He took his vorpal sword in hand:
      Long time the manxome foe he sought—
```"#.to_owned(),
            commit: commit.to_string(),
            path: path.to_owned(),
            original_start_line: Some(7),
            original_end_line: 8,
        };

        let expected = r#"diff --git a/poems/Jabberwocky.txt b/poems/Jabberwocky.txt
index 89840a2..06acdfc 100644
--- a/poems/Jabberwocky.txt
+++ b/poems/Jabberwocky.txt
@@ -4,7 +4,7 @@
      Beware the Jubjub bird, and shun
       The frumious Bandersnatch!’
 
-     He took his vorpal blade in hand:
-      Long time the manxome foe he sought--
+     He took his vorpal sword in hand:
+      Long time the manxome foe he sought—
      So rested he by the Tumtum tree,
       And stood awhile in thought.
"#;

        assert_eq!(
            suggestion.diff_with_repo(&repo).unwrap(),
            expected,
        );
    }

    #[test]
    fn suggestion_apply_to_writes_patch_to_writer() {
        use std::io::Cursor;


        let mut original_buffer = Vec::new();
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

        write!(original_buffer, "{}", original).unwrap();

        let suggestion = Suggestion {
            diff: "".to_owned(),
            comment: r#"``` suggestion
     He took his vorpal sword in hand:
      Long time the manxome foe he sought—
```"#.to_owned(),
            commit: "".to_owned(),
            path: "".to_owned(),
            original_start_line: Some(7),
            original_end_line: 8,
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

        let original_reader = Cursor::new(original_buffer);
        let mut actual = Cursor::new(Vec::new());
        suggestion.apply_to(original_reader, &mut actual).unwrap();

        assert_eq!(
            std::str::from_utf8(&actual.into_inner()).unwrap(),
            expected,
        );
    }
}
