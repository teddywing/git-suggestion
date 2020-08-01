use std::env;
use std::process;

use exitcode;

use github_suggestion::{Client, Suggestion, SuggestionUrl};
use github_suggestion_cli::gseprintln;
use github_suggestion_cli::config::Config;
use github_suggestion_cli::error::Error;
use github_suggestion_cli::is_suggestion_id;


fn main() {
    let args: Vec<_> = env::args().collect();

    let config = match Config::get(&args) {
        Ok(c) => c,
        Err(e) => {
            gseprintln!(e);

            process::exit(exitcode::DATAERR);
        },
    };

    if config.suggestions.is_empty() {
        process::exit(111);
    }

    let suggestions: Vec<Result<Suggestion, Error>> = config.suggestions
        .iter()
        .map(|s| {
            let suggestion = if is_suggestion_id(s)? {
                let client = Client::new(
                    &config.github_token,
                    &config.owner,
                    &config.repo,
                ).unwrap();

                client.fetch(&s).unwrap()
            } else {
                let url: SuggestionUrl = args[1].parse().unwrap();

                let client = Client::new(
                    &config.github_token,
                    &url.owner,
                    &url.repo,
                ).unwrap();

                client.fetch(&url.comment_id).unwrap()
            };

            Ok(suggestion)
        })
        .collect();

    let errors: Vec<&Error> = suggestions.iter()
        .filter(|r| r.is_err())

        // We know these `Results` are `Err`s.
        .map(|r| r.as_ref().err().unwrap())
        .collect();

    if !errors.is_empty() {
        for error in errors {
            eprintln!("error: {}", error);
        }

        return;
    }

    suggestions
        .iter()

        // We've already checked for `Err`s above.
        .map(|r| r.as_ref().unwrap())
        .for_each(|s| print!("{}", s.diff().unwrap()));
}
