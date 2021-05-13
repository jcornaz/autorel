use rstest::rstest;

use autorel_core::{BreakingInfo, Change, ChangeLog, ChangeType};

#[test]
pub fn is_empty_by_default() {
    let default = ChangeLog::default();

    assert!(default.breaking_changes.is_empty());
    assert!(default.features.is_empty());
    assert!(default.fixes.is_empty());
}

#[test]
pub fn adding_non_breaking_custom_type_change_has_no_effect() {
    assert_eq!(
        ChangeLog::default(),
        ChangeLog::default() + Change::new(ChangeType::Custom("testing"), "..."),
    );
}

#[rstest]
#[case(None)]
#[case(Some("main"))]
#[case(Some("core"))]
pub fn stores_features(#[case] scope: Option<&str>) {
    let mut change1 = Change::new(ChangeType::Feature, "Hello world!");
    change1.scope = scope;

    let mut change2 = Change::new(ChangeType::Feature, "Goodbye world!");
    change2.scope = scope;

    let changelog = ChangeLog::default() + change1.clone() + change2.clone();

    assert_eq!(
        changelog
            .features
            .get(&scope.map(String::from))
            .expect("Entry not added"),
        &vec![
            String::from(change1.description),
            String::from(change2.description)
        ],
    );
}

#[rstest]
#[case(None)]
#[case(Some("main"))]
#[case(Some("core"))]
pub fn stores_fixes(#[case] scope: Option<&str>) {
    let mut change1 = Change::new(ChangeType::Fix, "Hello world!");
    change1.scope = scope;

    let mut change2 = Change::new(ChangeType::Fix, "Goodbye world!");
    change2.scope = scope;

    let changelog = ChangeLog::default() + change1.clone() + change2.clone();

    assert_eq!(
        changelog
            .fixes
            .get(&scope.map(String::from))
            .expect("Entry not added"),
        &vec![
            String::from(change1.description),
            String::from(change2.description)
        ],
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

    assert_eq!(
        changelog
            .breaking_changes
            .get(&None)
            .expect("Entry not added"),
        &vec![String::from("oops...")],
    );
}
