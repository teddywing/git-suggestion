#![warn(rust_2018_idioms)]

pub mod config;
pub mod error;

mod arg;
mod owner_repo;
mod suggestion;


pub use suggestion::for_suggestion;
