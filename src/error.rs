use regex;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse regex")]
    Regex(#[from] regex::Error),
}


/// Print to standard error with a program-specific prefix.
#[macro_export]
macro_rules! gseprintln {
    ($arg:expr) => ({
        eprintln!("error: {}", $arg);
    })
}
