#![warn(rust_2018_idioms)]


pub mod client;
pub mod suggestion;

mod url;

pub use crate::client::Client;
pub use crate::suggestion::Suggestion;
pub use crate::url::SuggestionUrl;
