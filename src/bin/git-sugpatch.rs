use std::env;
use std::process;

use exitcode;

use github_suggestion::{Client, Suggestion, SuggestionUrl};
use github_suggestion_cli::{gseprintln, is_suggestion_id, owner_repo};
use github_suggestion_cli::config::Config;
use github_suggestion_cli::error::Error;


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

    // let mut owner_repo_error: owner_repo::Error;
    // let o_r = match config.o_r {
    //     Ok(o_r) => o_r,
    //     Err(e @ owner_repo::Error::NoRemote(_)) => owner_repo_error = e,
    //     Err(e) => {
    //         gseprintln!(e);
    //
    //         process::exit(111);
    //     },
    // };

    for suggestion_arg in config.suggestions {
        let suggestion = if match is_suggestion_id(&suggestion_arg) {
            Ok(p) => p,
            Err(e) => {
                gseprintln!(e);

                process::exit(exitcode::SOFTWARE);
            }
        } {
            // let o_r = match config.o_r {
            //     Ok(o_r) => o_r,
            //     Err(e) => {
            //         gseprintln!(e);
            //
            //         process::exit(exitcode::CONFIG);
            //     }
            // };
            // let o_r = match owner_repo {
            //     Ok(o_r) => o_r,
            //     Err(e) => {
            //         gseprintln!(e);
            //
            //         process::exit(exitcode::CONFIG);
            //     },
            // };
            // if let owner_repo::Error::NoRemote(e) = owner_repo_error {
            //     gseprintln!(e);
            //
            //     process::exit(exitcode::CONFIG);
            // }
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
            let url: SuggestionUrl = args[1].parse().unwrap();

            let client = Client::new(
                &config.github_token,
                &url.owner,
                &url.repo,
            ).unwrap();

            client.fetch(&url.comment_id).unwrap()
        };

        print!("{}", suggestion.diff().unwrap());
    }

    // let suggestions: Vec<Result<Suggestion, Error>> = config.suggestions
    //     .iter()
    //     .map(|s| {
    //         let suggestion = if is_suggestion_id(s)? {
    //             let o_r = owner_repo?;
    //
    //             let client = Client::new(
    //                 &config.github_token,
    //                 &o_r.owner,
    //                 &o_r.repo,
    //             ).unwrap();
    //
    //             client.fetch(&s).unwrap()
    //         } else {
    //             let url: SuggestionUrl = args[1].parse().unwrap();
    //
    //             let client = Client::new(
    //                 &config.github_token,
    //                 &url.owner,
    //                 &url.repo,
    //             ).unwrap();
    //
    //             client.fetch(&url.comment_id).unwrap()
    //         };
    //
    //         Ok(suggestion)
    //     })
    //     .collect();
    //
    // let errors: Vec<&Error> = suggestions.iter()
    //     .filter(|r| r.is_err())
    //
    //     // We know these `Results` are `Err`s.
    //     .map(|r| r.as_ref().err().unwrap())
    //     .collect();
    //
    // if !errors.is_empty() {
    //     for error in errors {
    //         eprintln!("error: {}", error);
    //     }
    //
    //     return;
    // }
    //
    // suggestions
    //     .iter()
    //
    //     // We've already checked for `Err`s above.
    //     .map(|r| r.as_ref().unwrap())
    //     .for_each(|s| print!("{}", s.diff().unwrap()));
}
