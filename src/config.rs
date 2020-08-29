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


use std::env;
use std::ffi::OsStr;
use std::process;

use getopts::{self, Options};
use git2::{self, Repository};
use thiserror::Error;

use crate::owner_repo::{self, OwnerRepo};
use crate::VERSION;


/// Program-specific prefix for Git config values.
const GIT_CONFIG_PREFIX: &'static str = "githubSuggestion.";

/// Configuration errors.
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

/// Configuration extracted from config files and command line arguments.
pub struct Config {
    pub github_token: String,
    pub o_r: Result<OwnerRepo, owner_repo::Error>,
    pub suggestions: Vec<String>,
}

impl Config {
    /// Set up command line arguments. Extract configuration values from command
    /// line arguments, Git config, and environment variables.
    pub fn get<S: AsRef<OsStr>>(args: &[S], usage_brief: &str) -> Result<Self, Error> {
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
        opts.optflag("V", "version", "show the program version");

        let opt_matches = opts.parse(&args[1..])?;

        if opt_matches.opt_present("h") {
            print_usage(&opts, usage_brief);

            process::exit(exitcode::USAGE);
        }

        if opt_matches.opt_present("V") {
            println!("{}", VERSION);

            process::exit(exitcode::OK);
        }

        if opt_matches.free.is_empty() {
            print_usage(&opts, usage_brief);

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
        })
    }

    /// Get a GitHub token, checking the following places in order:
    ///
    /// 1. Command line argument
    /// 2. Git config
    /// 3. Environment variable
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

    /// Get the Git remote name from the following places in order:
    ///
    /// 1. Command line argument
    /// 2. Git config
    ///
    /// If the value wasn't set, return `Ok(None)`.
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

/// Print command line usage information to standard output.
fn print_usage(opts: &Options, brief: &str) {
    print!("{}", opts.usage(brief));
}

/// Build a Git config key using the program-specific prefix and a subkey.
fn git_config_key(key: &str) -> String {
    format!("{}.{}", GIT_CONFIG_PREFIX, key)
}
