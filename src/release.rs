#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Release<V> {
    pub prev_version: Option<V>,
    pub version: V,
}
