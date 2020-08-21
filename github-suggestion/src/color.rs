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


use ansi_term::Style;
use colorparse;
use git2;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Git(#[from] git2::Error),
}


static SLOTS: [&'static str; 6] = [
    "commit",
    "context",
    "frag",
    "func",
    "old",
    "new",
];


#[derive(Debug, Default)]
pub struct Diff {
    context: Option<Style>,
    meta: Option<Style>,
    frag: Option<Style>,
    func: Option<Style>,
    old: Option<Style>,
    new: Option<Style>,
    commit: Option<Style>,
    whitespace: Option<Style>,
    oldMoved: Option<Style>,
    newMoved: Option<Style>,
    oldMovedDimmed: Option<Style>,
    oldMovedAlternative: Option<Style>,
    oldMovedAlternativeDimmed: Option<Style>,
    newMovedDimmed: Option<Style>,
    newMovedAlternative: Option<Style>,
    newMovedAlternativeDimmed: Option<Style>,
}


// pub fn diff(config: &git2::Config) -> Diff {
//     let diff = Diff::default();
//
//     for slot in SLOTS {
//         let colors = match config.get_string(slot) {
//             Ok(c) => c,
//             Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(None),
//             Err(e) => return Err(Error::Git(e)),
//         };
//
//         colorparse::parse(colors)?;
//     }
// }

impl Diff {
    pub fn from_config(config: &git2::Config) -> Result<Self, Error> {
        let diff = Self::default();

        for slot in &SLOTS {
            let colors = match config.get_string(slot) {
                Ok(c) => c,
                Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(None),
                Err(e) => return Err(Error::Git(e)),
            };

            colorparse::parse(&colors)?;
        }

        Ok(diff)
    }
}
