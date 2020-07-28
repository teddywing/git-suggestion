use std::env;
use std::process;

use github_suggestion::{Client, SuggestionUrl};


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        process::exit(111);
    }

    let url: SuggestionUrl = args[1].parse().unwrap();

    let client = Client::new(
        env!("GITHUB_TOKEN"),
        &url.owner,
        &url.repo,
    ).unwrap();

    let suggestion = client.fetch(&url.comment_id).unwrap();

    print!("{}", suggestion.diff().unwrap());
}
