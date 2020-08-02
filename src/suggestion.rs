use std::process;

use exitcode;

use github_suggestion::{Client, Suggestion, SuggestionUrl};

use crate::gseprintln;
use crate::arg::is_suggestion_id;
use crate::config::Config;


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

            let client = Client::new(
                &config.github_token,
                &o_r.owner,
                &o_r.repo,
            ).unwrap();

            client.fetch(&suggestion_arg).unwrap()
        } else {
            let url: SuggestionUrl = suggestion_arg.parse().unwrap();

            let client = Client::new(
                &config.github_token,
                &url.owner,
                &url.repo,
            ).unwrap();

            client.fetch(&url.comment_id).unwrap()
        };

        f(&suggestion);
    }
}
