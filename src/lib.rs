#![warn(rust_2018_idioms)]

pub mod config;
pub mod error;

pub(crate) mod owner_repo;

mod arg;


pub use arg::is_suggestion_id;
