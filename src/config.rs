use std::env;

use getopts::{self, Options};
use git2::{self, Repository};
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
    pub o_r: Result<OwnerRepo, owner_repo::Error>,
    pub suggestions: Vec<String>,
}

impl Config {
    // fn from_args(args: &env::Args) -> Result<Self, Error> {
    pub fn get(args: &[String]) -> Result<Self, Error> {
        let mut opts = Options::new();

        opts.optopt("", "github-token", "", "TOKEN");
        opts.optopt("", "remote", "", "REMOTE");

        let opt_matches = opts.parse(&args[1..])?;

        let git_config = Repository::open(".")?.config()?;

        let o_r = OwnerRepo::from_remote(
            Self::remote(&opt_matches, &git_config)?.as_deref(),
        );

        Ok(Config {
            github_token: Self::github_token(&opt_matches, &git_config)?,
            o_r: o_r,
            suggestions: opt_matches.free,
        })
    }

    fn github_token(
        opt_matches: &getopts::Matches,
        git_config: &git2::Config,
    ) -> Result<String, Error> {
        match opt_matches.opt_str("github-token") {
            Some(t) => Ok(t),
            None =>
                match git_config.get_string(&git_config_key("githubToken")) {
                    Err(e) if e.code() == git2::ErrorCode::NotFound =>
                        env::var("GITHUB_TOKEN")
                            .map_err(|e| Error::EnvVar(e)),
                    r => r.map_err(|e| Error::Git(e)),
                },
        }
    }

    fn remote(
        opt_matches: &getopts::Matches,
        git_config: &git2::Config,
    ) -> Result<Option<String>, git2::Error> {
        match opt_matches.opt_str("remote") {
            Some(r) => Ok(Some(r)),
            None => match git_config.get_string(&git_config_key("remote")) {
                Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
                r => r.map(|r| Some(r)),
            },
        }
    }
}

fn git_config_key(key: &str) -> String {
    format!("{}.{}", GIT_CONFIG_PREFIX, key)
}
