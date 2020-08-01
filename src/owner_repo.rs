use std::str::FromStr;

use git2::Repository;
use thiserror::Error;
use url;
use url::Url;


#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Git(#[from] git2::Error),

    #[error(transparent)]
    OwnerRepo(#[from] OwnerRepoError),

    #[error("Unable to find remote '{0}'")]
    NoRemote(String),
}

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

impl FromStr for OwnerRepo {
    type Err = OwnerRepoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        let path = url.path_segments()
            .ok_or(OwnerRepoError::NoPath)?
            .collect::<Vec<_>>();

        if path.len() < 2 {
            return Err(OwnerRepoError::NoOwnerRepo);
        }

        Ok(OwnerRepo {
            owner: path[0].to_owned(),
            repo: path[1].to_owned(),
        })
    }
}

impl OwnerRepo {
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
}
