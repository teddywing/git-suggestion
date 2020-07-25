use std::str::FromStr;

use thiserror::Error;

use url;
use url::Url;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse URL")]
    Url(#[from] url::ParseError),

    #[error("URL has no path")]
    NoPath,

    #[error("URL has no fragment")]
    NoFragment,
}

#[derive(Debug)]
pub struct SuggestionUrl {
    pub owner: String,
    pub repo: String,
    pub comment_id: String,
}

impl FromStr for SuggestionUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        let path = url.path_segments()
            .ok_or(Error::NoPath)?
            .collect::<Vec<_>>();

        Ok(SuggestionUrl {
            owner: path[0].to_owned(),
            repo: path[1].to_owned(),
            comment_id: url.fragment()
                .ok_or(Error::NoFragment)?
                .replacen("discussion_r", "", 1),
        })
    }
}
