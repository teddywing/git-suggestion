use std::env;
use std::process;

use exitcode;

use github_suggestion::{Client, SuggestionUrl};
use github_suggestion_cli::{gseprintln, is_suggestion_id};
use github_suggestion_cli::config::Config;


fn main() {
    let args: Vec<_> = env::args().collect();

    let config = match Config::get(&args) {
        Ok(c) => c,
        Err(e) => {
            gseprintln!(e);

            process::exit(exitcode::CONFIG);
        },
    };

    if config.suggestions.is_empty() {
        process::exit(111);
    }

    for suggestion_arg in config.suggestions {
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

        print!("{}", suggestion.diff().unwrap());
    }
}
