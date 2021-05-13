use std::collections::HashMap;
use std::ops::{Add, AddAssign};

use crate::{BreakingInfo, Change, ChangeType, SemverScope};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ChangeLog {
    pub breaking_changes: Section,
    pub features: Section,
    pub fixes: Section,
}

pub type Section = HashMap<Option<String>, Vec<String>>;

impl ChangeLog {
    pub fn is_empty(&self) -> bool {
        self.breaking_changes.is_empty() && self.features.is_empty() && self.fixes.is_empty()
    }

    pub fn semver_scope(&self) -> Option<SemverScope> {
        if !self.breaking_changes.is_empty() {
            Some(SemverScope::Breaking)
        } else if !self.features.is_empty() {
            Some(SemverScope::Feature)
        } else if !self.fixes.is_empty() {
            Some(SemverScope::Fix)
        } else {
            None
        }
    }
}

impl AddAssign<Change<'_>> for ChangeLog {
    fn add_assign(&mut self, change: Change<'_>) {
        match change.breaking {
            BreakingInfo::NotBreaking => (),
            BreakingInfo::Breaking => {
                append(&mut self.breaking_changes, change.scope, change.description)
            }
            BreakingInfo::BreakingWithDescription(info) => {
                append(&mut self.breaking_changes, change.scope, info)
            }
        }

        let section = match change.type_ {
            ChangeType::Fix => &mut self.fixes,
            ChangeType::Feature => &mut self.features,
            ChangeType::Custom(_) => return,
        };

        append(section, change.scope, change.description);
    }
}

impl Add<Change<'_>> for ChangeLog {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Change<'_>) -> Self::Output {
        self += rhs;
        self
    }
}

fn append(section: &mut Section, scope: Option<&str>, value: &str) {
    section
        .entry(scope.map(String::from))
        .or_default()
        .push(value.to_owned());
}
