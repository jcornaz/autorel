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

#[test]
pub fn stores_feature_without_scope() {
    let description1 = "Hello world!";
    let description2 = "Goodbye world!";
    let mut changelog = ChangeLog::default();
    changelog += Change::new(ChangeType::Feature, description1);
    changelog += Change::new(ChangeType::Feature, description2);

    assert_eq!(
        changelog.features.get(&None).expect("Entry not added"),
        &vec![String::from(description1), String::from(description2)],
    );
}

#[test]
pub fn stores_fix_without_scope() {
    let description1 = "Hello world!";
    let description2 = "Goodbye world!";
    let mut changelog = ChangeLog::default();
    changelog += Change::new(ChangeType::Fix, description1);
    changelog += Change::new(ChangeType::Fix, description2);

    assert_eq!(
        changelog.fixes.get(&None).expect("Entry not added"),
        &vec![String::from(description1), String::from(description2)],
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
