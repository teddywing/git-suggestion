use std::env;
use std::process;

use exitcode;

use github_suggestion_cli::{gseprintln, for_suggestion};
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
        print!(
            "{}",
            config.usage("usage: git sugpatch [options] <suggestion>..."),
        );

        process::exit(exitcode::USAGE);
    }

    for_suggestion(
        &config,
        |suggestion| {
            let diff = match suggestion.diff() {
                Ok(d) => d,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
            };

            print!("{}", diff);
        },
    );
}
