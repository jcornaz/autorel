use autorel_chlg::ChangeLog;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Release<V> {
    pub prev_version: Option<V>,
    pub version: V,
    pub changelog: ChangeLog,
}
