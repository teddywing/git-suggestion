use std::env;

use getopts::Options;
use git2::Repository;
use thiserror::Error;

use crate::owner_repo::{self, OwnerRepo};


const GIT_CONFIG_PREFIX: &'static str = "githubSuggestion.";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse arguments")]
    Opts(#[from] getopts::Fail),

    #[error("Error getting environment variable")]
    EnvVar(#[from] env::VarError),

    #[error(transparent)]
    OwnerRepo(#[from] owner_repo::Error),

    #[error(transparent)]
    Git(#[from] git2::Error),
}

#[derive(Debug)]
pub struct Config {
    pub github_token: String,
    pub owner: String,
    pub repo: String,
}

impl Config {
    // fn from_args(args: &env::Args) -> Result<Self, Error> {
    pub fn get(args: &[String]) -> Result<Self, Error> {
        let mut opts = Options::new();

        opts.optopt("", "github-token", "", "TOKEN");
        opts.optopt("", "remote", "", "REMOTE");

        let opt_matches = opts.parse(&args[1..])?;

        let git_config = Repository::open(".")?.config()?;

        let github_token = match opt_matches.opt_str("github-token") {
            Some(t) => Ok(t),
            None =>
                match git_config.get_string(&git_config_key("githubToken")) {
                    Err(e) if e.code() == git2::ErrorCode::NotFound =>
                        env::var("GITHUB_TOKEN")
                            .map_err(|e| Error::EnvVar(e)),
                    r => r.map_err(|e| Error::Git(e)),
                },
        }?;

        let remote = match opt_matches.opt_str("remote") {
            Some(r) => Ok(Some(r)),
            None => match git_config.get_string(&git_config_key("remote")) {
                Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
                r => r.map(|r| Some(r)),
            },
        }?;

        let o_r = OwnerRepo::from_remote(remote.as_deref())?;

        Ok(Config {
            github_token,
            owner: o_r.owner,
            repo: o_r.repo,
        })
    }
}

fn git_config_key(key: &str) -> String {
    format!("{}.{}", GIT_CONFIG_PREFIX, key)
}
