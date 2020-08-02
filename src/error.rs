use regex;
use thiserror::Error;

use crate::owner_repo;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse regex")]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    NoRemote(#[from] owner_repo::Error),
}


#[macro_export]
macro_rules! gseprintln {
    ($arg:expr) => ({
        eprintln!("error: {}", $arg);
    })
}
