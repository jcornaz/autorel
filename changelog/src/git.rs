use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::FromStr;

use super::{Change, ChangeLog};

pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self {
            repo: git2::Repository::open(path)?,
        })
    }

    pub fn find_latest_release<V: FromStr + Ord>(
        &self,
        tag_prefix: &str,
    ) -> Result<Option<V>, Error> {
        Ok(self
            .repo
            .tag_names(Some(&(String::from(tag_prefix) + "*")))?
            .iter()
            .filter_map(|tag| {
                tag.and_then(|it| it.strip_prefix(tag_prefix))
                    .and_then(|it| it.parse().ok())
            })
            .max())
    }

    pub fn load_changelog(&self, from: Option<&str>) -> Result<ChangeLog, Error> {
        let mut walker = self.repo.revwalk()?;
        walker.push_head()?;
        if let Some(from) = from {
            walker.push_range(&(String::from(from) + "..HEAD"))?;
        }

        let mut result = ChangeLog::default();

        for oid in walker {
            if let Some(change) = self
                .repo
                .find_commit(oid?)?
                .message()
                .and_then(Change::parse_conventional_commit)
            {
                result += change;
            }
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub struct Error(git2::Error);

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Self(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot read git history: {}", self.0)
    }
}

impl std::error::Error for Error {}
