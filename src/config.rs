use std::env;
use std::process;

use getopts::{self, Options};
use git2::{self, Repository};
use thiserror::Error;

use crate::owner_repo::{self, OwnerRepo};


const GIT_CONFIG_PREFIX: &'static str = "githubSuggestion.";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse arguments: {0}")]
    Opts(#[from] getopts::Fail),

    #[error("Error getting environment variable '{var}'")]
    EnvVar {
        source: env::VarError,
        var: String,
    },

    #[error(transparent)]
    OwnerRepo(#[from] owner_repo::Error),

    #[error(transparent)]
    Git(#[from] git2::Error),
}

pub struct Config<'a> {
    pub github_token: String,
    pub o_r: Result<OwnerRepo, owner_repo::Error>,
    pub suggestions: Vec<String>,

    opts: Options,
    usage_brief: &'a str,
}

impl<'a> Config<'a> {
    pub fn get(args: &[String], usage_brief: &'a str) -> Result<Self, Error> {
        let mut opts = Options::new();

        opts.optopt(
            "",
            "github-token",
            r#"GitHub API token with "repo" permission"#,
            "TOKEN",
        );
        opts.optopt(
            "",
            "remote",
            "remote name, defaults to 'origin'",
            "REMOTE",
        );
        opts.optflag("h", "help", "print this help menu");

        let opt_matches = opts.parse(&args[1..])?;

        if opt_matches.opt_present("h") {
            print!("{}", opts.usage(&usage_brief));

            process::exit(exitcode::USAGE);
        }

        let git_config = Repository::open(".")?.config()?;

        let o_r = OwnerRepo::from_remote(
            Self::remote(&opt_matches, &git_config)?.as_deref(),
        );

        Ok(Config {
            github_token: Self::github_token(&opt_matches, &git_config)?,
            o_r: o_r,
            suggestions: opt_matches.free,

            opts: opts,
            usage_brief,
        })
    }

    pub fn print_usage(&self) {
        print!("{}", self.opts.usage(&self.usage_brief))
    }

    fn github_token(
        opt_matches: &getopts::Matches,
        git_config: &git2::Config,
    ) -> Result<String, Error> {
        match opt_matches.opt_str("github-token") {
            Some(t) => Ok(t),
            None =>
                match git_config.get_string(&git_config_key("githubToken")) {
                    Err(e) if e.code() == git2::ErrorCode::NotFound => {
                        let key = "GITHUB_TOKEN";

                        env::var(key)
                            .map_err(|e| Error::EnvVar {
                                source: e,
                                var: key.to_owned(),
                            })
                    },
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
