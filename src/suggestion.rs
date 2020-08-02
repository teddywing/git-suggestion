use std::process;

use exitcode;

use github_suggestion::{Client, Suggestion, SuggestionUrl};

use crate::gseprintln;
use crate::arg::is_suggestion_id;
use crate::config::Config;


/// For all suggestions in `config.suggestions`, fetch the suggestion from the
/// API and call `f` with it.
pub fn for_suggestion<F>(config: &Config, f: F)
where F: Fn(&Suggestion)
{
    for suggestion_arg in &config.suggestions {
        let suggestion = if match is_suggestion_id(&suggestion_arg) {
            Ok(p) => p,
            Err(e) => {
                gseprintln!(e);

                process::exit(exitcode::SOFTWARE);
            }
        } {
            let o_r = match &config.o_r {
                Ok(o_r) => o_r,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::CONFIG);
                },
            };

            let client = match Client::new(
                &config.github_token,
                &o_r.owner,
                &o_r.repo,
            ) {
                Ok(c) => c,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::SOFTWARE);
                },
            };

            match client.fetch(&suggestion_arg) {
                Ok(s) => s,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
            }
        } else {
            let url: SuggestionUrl = match suggestion_arg.parse() {
                Ok(u) => u,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::USAGE);
                },
            };

            let client = match Client::new(
                &config.github_token,
                &url.owner,
                &url.repo,
            ) {
                Ok(c) => c,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::SOFTWARE);
                },
            };

            match client.fetch(&url.comment_id) {
                Ok(s) => s,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
            }
        };

        f(&suggestion);
    }
}
