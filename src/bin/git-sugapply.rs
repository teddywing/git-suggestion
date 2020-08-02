use std::env;
use std::process;

use exitcode;

use github_suggestion_cli::{gseprintln, for_suggestion};
use github_suggestion_cli::config::Config;


fn main() {
    let args: Vec<_> = env::args().collect();

    let config = match Config::get(
        &args,
        "usage: git sugapply [options] <suggestion>...",
    ) {
        Ok(c) => c,
        Err(e) => {
            gseprintln!(e);

            process::exit(exitcode::CONFIG);
        },
    };

    if config.suggestions.is_empty() {
        config.print_usage();

        process::exit(exitcode::USAGE);
    }

    for_suggestion(
        &config,
        |suggestion| {
            match suggestion.apply() {
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
                _ => (),
            }
        },
    );
}
