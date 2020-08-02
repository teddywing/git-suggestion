use std::env;
use std::process;

use exitcode;

use github_suggestion_cli::{gseprintln, for_suggestion};
use github_suggestion_cli::config::Config;


fn main() {
    let args: Vec<_> = env::args().collect();

    let config = match Config::get(
        &args,
        "usage: git sugpatch [options] <suggestion>...",
    ) {
        Ok(c) => c,
        Err(e) => {
            gseprintln!(e);

            process::exit(exitcode::CONFIG);
        },
    };

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
