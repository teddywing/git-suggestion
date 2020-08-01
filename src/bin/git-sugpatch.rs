use std::env;
use std::process;

use github_suggestion::{Client, SuggestionUrl};
use github_suggestion_cli::config::Config;


fn main() {
    let args: Vec<_> = env::args().collect();

    let config = Config::get(&args).unwrap();

    if args.len() < 2 {
        process::exit(111);
    }

    let url: SuggestionUrl = args[1].parse().unwrap();

    let client = Client::new(
        &config.github_token,
        &config.owner,
        &config.repo,
    ).unwrap();

    let suggestion = client.fetch(&url.comment_id).unwrap();

    print!("{}", suggestion.diff().unwrap());
}
