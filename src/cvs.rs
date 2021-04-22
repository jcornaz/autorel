use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::FromStr;

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

    pub fn find_change_scope<'a, S: Default + From<Commit<'a>> + Ord>(
        &'a self,
        tag: &str,
    ) -> Result<S, Error> {
        let mut walker = self.repo.revwalk()?;
        walker.push_head()?;
        walker.push_range(&(String::from(tag) + "..HEAD"))?;

        let mut result = S::default();

        for oid in walker {
            let commit: Commit<'_> = self.repo.find_commit(oid?)?.into();
            let scope: S = commit.into();
            if scope > result {
                result = scope;
            }
        }

        Ok(result)
    }
}

pub struct Commit<'a>(git2::Commit<'a>);

impl Commit<'_> {
    pub fn message(&self) -> Option<&str> {
        self.0.message()
    }
}

impl<'a> From<git2::Commit<'a>> for Commit<'a> {
    fn from(c: git2::Commit<'a>) -> Self {
        Self(c)
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
