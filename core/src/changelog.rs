use std::ops::{Add, AddAssign};

use crate::Change;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ChangeLog {
    pub breaking_changes: Vec<Section>,
    pub features: Vec<Section>,
    pub fixes: Vec<Section>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Section {
    pub scope: Option<String>,
    pub entries: Vec<String>,
}

impl AddAssign<Change<'_>> for ChangeLog {
    fn add_assign(&mut self, rhs: Change<'_>) {}
}

impl Add<Change<'_>> for ChangeLog {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Change<'_>) -> Self::Output {
        self += rhs;
        self
    }
}
