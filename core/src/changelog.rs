use std::collections::HashMap;
use std::ops::{Add, AddAssign};

use crate::{BreakingInfo, Change, ChangeType};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ChangeLog {
    pub breaking_changes: Section,
    pub features: Section,
    pub fixes: Section,
}

pub type Section = HashMap<Option<String>, Vec<String>>;

impl AddAssign<Change<'_>> for ChangeLog {
    fn add_assign(&mut self, change: Change<'_>) {
        match change.breaking {
            BreakingInfo::NotBreaking => (),
            BreakingInfo::Breaking => append(&mut self.breaking_changes, change.description),
            BreakingInfo::BreakingWithDescription(info) => append(&mut self.breaking_changes, info),
        }

        let section = match change.type_ {
            ChangeType::Fix => &mut self.fixes,
            ChangeType::Feature => &mut self.features,
            ChangeType::Custom(_) => return,
        };

        append(section, change.description);
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

fn append(section: &mut Section, value: &str) {
    section.entry(None).or_default().push(value.to_owned());
}
