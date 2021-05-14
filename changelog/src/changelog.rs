use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

use crate::{BreakingInfo, Change, ChangeType, SemverScope};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ChangeLog {
    breaking_changes: Section,
    features: Section,
    fixes: Section,
    scopes: HashSet<Option<Scope>>,
}

pub(crate) type Scope = String;
pub(crate) type Section = HashMap<Option<Scope>, Vec<String>>;

impl AddAssign<Change<'_>> for ChangeLog {
    fn add_assign(&mut self, change: Change<'_>) {
        let scope = change.scope.map(String::from);

        match change.breaking {
            BreakingInfo::NotBreaking => (),
            BreakingInfo::Breaking => Self::append(
                &mut self.scopes,
                &mut self.breaking_changes,
                scope.clone(),
                change.description,
            ),
            BreakingInfo::BreakingWithDescription(info) => Self::append(
                &mut self.scopes,
                &mut self.breaking_changes,
                scope.clone(),
                info,
            ),
        }

        let section = match change.type_ {
            ChangeType::Fix => &mut self.fixes,
            ChangeType::Feature => &mut self.features,
            ChangeType::Custom(_) => return,
        };

        Self::append(&mut self.scopes, section, scope, change.description);
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

impl ChangeLog {
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

    pub(crate) fn scopes(&self) -> impl Iterator<Item = &Option<Scope>> {
        self.scopes.iter()
    }

    pub(crate) fn breaking_changes(&self) -> &Section {
        &self.breaking_changes
    }

    pub(crate) fn features(&self) -> &Section {
        &self.features
    }

    pub(crate) fn fixes(&self) -> &Section {
        &self.fixes
    }

    fn append(
        scopes: &mut HashSet<Option<Scope>>,
        section: &mut Section,
        scope: Option<Scope>,
        value: &str,
    ) {
        if !scopes.contains(&scope) {
            scopes.insert(scope.clone());
        }
        section.entry(scope).or_default().push(value.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    pub fn is_empty_by_default() {
        let default = ChangeLog::default();

        assert!(default.semver_scope().is_none());
        assert!(default.breaking_changes().is_empty());
        assert!(default.features().is_empty());
        assert!(default.fixes().is_empty());
        assert!(default.scopes().collect::<Vec<_>>().is_empty());
    }

    #[test]
    pub fn adding_non_breaking_custom_type_change_has_no_effect() {
        let changelog = ChangeLog::default() + Change::new(ChangeType::Custom("testing"), "...");

        assert!(changelog.semver_scope().is_none());
        assert_eq!(ChangeLog::default(), changelog);
    }

    #[rstest]
    #[case(None)]
    #[case(Some("main"))]
    #[case(Some("changelog"))]
    pub fn stores_features(#[case] scope: Option<&str>) {
        let change1 = Change {
            scope,
            ..Change::new(ChangeType::Feature, "Hello world!")
        };

        let change2 = Change {
            scope,
            ..Change::new(ChangeType::Feature, "Goodbye world!")
        };

        let changelog = ChangeLog::default() + change1.clone() + change2.clone();

        assert_eq!(
            changelog.scopes().cloned().collect::<Vec<_>>(),
            vec![scope.map(String::from)]
        );
        assert_eq!(changelog.semver_scope(), Some(SemverScope::Feature));
        assert_eq!(
            changelog
                .features()
                .get(&scope.map(String::from))
                .expect("Entry not added"),
            &vec![change1.description, change2.description],
        );
    }

    #[rstest]
    #[case(None)]
    #[case(Some("main"))]
    #[case(Some("changelog"))]
    pub fn stores_fixes(#[case] scope: Option<&str>) {
        let mut change1 = Change::new(ChangeType::Fix, "Hello world!");
        change1.scope = scope;

        let mut change2 = Change::new(ChangeType::Fix, "Goodbye world!");
        change2.scope = scope;

        let changelog = ChangeLog::default() + change1.clone() + change2.clone();

        assert_eq!(
            changelog.scopes().cloned().collect::<Vec<_>>(),
            vec![scope.map(String::from)]
        );
        assert_eq!(changelog.semver_scope(), Some(SemverScope::Fix));
        assert_eq!(
            changelog
                .fixes()
                .get(&scope.map(String::from))
                .expect("Entry not added"),
            &vec![change1.description, change2.description],
        );
    }

    #[rstest]
    #[case(ChangeType::Feature)]
    #[case(ChangeType::Fix)]
    #[case(ChangeType::Custom("test"))]
    fn appends_description_to_breaking_changes_if_there_is_no_more_info(#[case] type_: ChangeType) {
        let description = "Hello world!";
        let mut change = Change::new(type_, description);
        change.breaking = BreakingInfo::Breaking;

        let changelog = ChangeLog::default() + change;

        assert_eq!(changelog.semver_scope(), Some(SemverScope::Breaking));
        assert_eq!(
            changelog
                .breaking_changes
                .get(&None)
                .expect("Entry not added"),
            &vec![String::from(description)],
        );
    }

    #[rstest]
    #[case(ChangeType::Feature)]
    #[case(ChangeType::Fix)]
    #[case(ChangeType::Custom("test"))]
    fn breaking_changes_info_are_appended_to_breaking_changes(#[case] type_: ChangeType) {
        let description = "Hello world!";
        let mut change = Change::new(type_, description);
        change.breaking = BreakingInfo::BreakingWithDescription("oops...");

        let changelog = ChangeLog::default() + change;

        assert_eq!(changelog.semver_scope(), Some(SemverScope::Breaking));
        assert_eq!(
            changelog
                .breaking_changes
                .get(&None)
                .expect("Entry not added"),
            &vec![String::from("oops...")],
        );
    }
}
