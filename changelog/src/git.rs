use git2::Sort;

use super::{Change, ChangeLog};

pub trait ChangeLogRepository {
    type Error: std::error::Error;
    fn load_changelog(&self, from: Option<&str>) -> Result<ChangeLog, Self::Error>;
}

impl ChangeLogRepository for git2::Repository {
    type Error = git2::Error;

    fn load_changelog(&self, from: Option<&str>) -> Result<ChangeLog, Self::Error> {
        let mut walker = self.revwalk()?;
        walker.push_head()?;
        let _ = walker.set_sorting(Sort::REVERSE);

        if let Some(from) = from {
            walker.push_range(&(String::from(from) + "..HEAD"))?;
        }

        let mut result = ChangeLog::default();

        for oid in walker {
            if let Some(change) = self
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
