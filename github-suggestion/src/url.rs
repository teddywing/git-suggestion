// Copyright (c) 2020  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use std::str::FromStr;

use thiserror::Error;

use url;
use url::Url;


/// Errors parsing a suggestion URL.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse URL")]
    Url(#[from] url::ParseError),

    #[error("URL has no path")]
    NoPath,

    #[error("URL has no fragment")]
    NoFragment,

    #[error("Unable to parse owner or repo")]
    NoOwnerRepo,
}

/// The important parts of a suggestion comment URL.
#[derive(Debug)]
pub struct SuggestionUrl {
    pub owner: String,
    pub repo: String,
    pub comment_id: String,
}

/// Parses a URL with the format
/// `https://github.com/teddywing/github-suggestion/pull/1#discussion_r459691747`.
impl FromStr for SuggestionUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        let path = url.path_segments()
            .ok_or(Error::NoPath)?
            .collect::<Vec<_>>();

        if path.len() < 2 {
            return Err(Error::NoOwnerRepo);
        }

        Ok(SuggestionUrl {
            owner: path[0].to_owned(),
            repo: path[1].to_owned(),
            comment_id: url.fragment()
                .ok_or(Error::NoFragment)?
                .replacen("discussion_r", "", 1),
        })
    }
}
