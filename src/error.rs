use regex;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse regex")]
    Regex(#[from] regex::Error),
}


#[macro_export]
macro_rules! gseprintln {
    ($arg:expr) => ({
        eprintln!("error: {}", $arg);
    })
}