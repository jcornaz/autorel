use autorel_core::{Change, ChangeLog, ChangeType};

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
