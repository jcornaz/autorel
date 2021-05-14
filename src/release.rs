use git2::Repository;

use autorel_chlg::ChangeLog;

pub struct Release<V> {
    pub prev_version: Option<V>,
    pub version: V,
    pub changelog: ChangeLog,
    pub repo: Repository,
}
