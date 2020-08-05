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

use git2::Repository;
use thiserror::Error;
use url;
use url::Url;


/// Errors getting an `OwnerRepo` from a remote in a Git repository.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Git(#[from] git2::Error),

    #[error(transparent)]
    OwnerRepo(#[from] OwnerRepoError),

    #[error("Unable to find remote '{0}'")]
    NoRemote(String),
}

/// Errors parsing an `OwnerRepo`.
#[derive(Debug, Error)]
pub enum OwnerRepoError {
    #[error("Unable to parse URL")]
    Url(#[from] url::ParseError),

    #[error("URL has no path")]
    NoPath,

    #[error("Unable to parse owner or repo")]
    NoOwnerRepo,
}


#[derive(Debug)]
pub struct OwnerRepo {
    pub owner: String,
    pub repo: String,
}

/// Parse an owner-repo pair from a Git remote. Can be either an HTTP URL
/// (`https://github.com/teddywing/git-suggestion.git`) or an SSH-style
/// reference (`git@github.com:teddywing/git-suggestion.git`).
impl FromStr for OwnerRepo {
    type Err = OwnerRepoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = match Url::parse(s) {
            Err(url::ParseError::RelativeUrlWithoutBase) =>
                return OwnerRepo::from_ssh(s),

            r => r,
        }?;
        let path = url.path_segments()
            .ok_or(OwnerRepoError::NoPath)?
            .collect::<Vec<_>>();

        if path.len() < 2 {
            return Err(OwnerRepoError::NoOwnerRepo);
        }

        let repo = path[1]
            .strip_suffix(".git")
            .unwrap_or(path[1]);

        Ok(OwnerRepo {
            owner: path[0].to_owned(),
            repo: repo.to_owned(),
        })
    }
}

impl OwnerRepo {
    /// Parse an `OwnerRepo` from the URL for `remote_name` in the current
    /// repository.
    pub fn from_remote(
        remote_name: Option<&str>,
    ) -> Result<OwnerRepo, Error> {
        let repo = Repository::open(".")?;

        let remote_name = match remote_name {
            Some(r) => r,
            None => "origin",
        };

        let remote = repo.find_remote(remote_name)?;
        let url = remote.url()
            .ok_or_else(|| Error::NoRemote(remote_name.to_owned()))?;

        Ok(url.parse()?)
    }

    /// Parse an `OwnerRepo` from an SSH-style reference
    /// (`git@github.com:teddywing/git-suggestion.git`).
    pub fn from_ssh(ssh: &str) -> Result<Self, OwnerRepoError> {
        let address_path: Vec<_> = ssh.splitn(2, ':').collect();
        let path = address_path.get(1)
            .ok_or(OwnerRepoError::NoOwnerRepo)?;

        let path = path
            .strip_suffix(".git")
            .unwrap_or(path);

        let segments: Vec<_> = path.split('/').collect();

        if segments.len() < 2 {
            return Err(OwnerRepoError::NoOwnerRepo);
        }

        Ok(OwnerRepo {
            owner: segments[0].to_owned(),
            repo: segments[1].to_owned(),
        })
    }
}
