use rstest::rstest;

use autorel_core::{BreakingInfo, Change, ChangeLog, ChangeType, SemverScope};

fn semver_scope_of(message: &str) -> Option<SemverScope> {
    let change = Change::parse_conventional_commit(message)?;
    let changelog = ChangeLog::default() + change;
    changelog.semver_scope()
}

#[test]
fn can_parse_empty_commit_message() {
    assert!(Change::parse_conventional_commit("").is_none())
}

#[rstest]
#[case("")]
#[case("Hello world")]
#[case("Hello world\n\nwith multiple lines")]
fn returns_none_for_non_conventional_commits(#[case] message: &str) {
    assert_eq!(None, semver_scope_of(message));
}

#[rstest]
#[case("feat!: Hello world")]
#[case("feat(withscope)!: Hello world")]
#[case("fix!: Hello world")]
#[case("other(withscope)!: Hello world")]
#[case("feat: Hello world\n\nBREAKING CHANGE: This is breaking")]
#[case("other: Hello world\n\nBREAKING CHANGE: This is breaking")]
fn recognize_breaking_changes(#[case] message: &str) {
    let scope = semver_scope_of(message);
    assert!(matches!(scope, Some(SemverScope::Breaking)));
}

#[rstest]
#[case("feat: Hello world")]
#[case("feat: a\n\n")]
#[case("feat(withscope): Hello world")]
#[case("feat: Hello world\n\nwith multiple lines")]
fn recognize_feature(#[case] message: &str) {
    let scope = semver_scope_of(message);
    assert!(matches!(scope, Some(SemverScope::Feature)));
}

#[rstest]
#[case("fix: Hello world")]
#[case("fix(withscope): Hello world")]
#[case("fix: Hello world\n\nwith multiple lines")]
fn recognize_fix(#[case] message: &str) {
    let scope = semver_scope_of(message);
    assert!(matches!(scope, Some(SemverScope::Fix)));
}

#[rstest]
#[case("chore: Hello world")]
#[case("chore: Hello world!")]
#[case("chore: Hello !: world")]
#[case("featuring: a")]
#[case("tests(withscope): Hello world")]
#[case("refactor: Hello world\n\nwith multiple lines")]
fn recognize_internal_changes(#[case] message: &str) {
    let scope = semver_scope_of(message);
    assert_eq!(None, scope);
}

#[rstest]
#[case("fix: coucou", ChangeType::Fix)]
#[case("fix!: coucou", ChangeType::Fix)]
#[case("fix(withscope): c'est moi", ChangeType::Fix)]
#[case("feat: c'est moi", ChangeType::Feature)]
#[case("feat(withscope)!: c'est moi", ChangeType::Feature)]
#[case("mytype(withscope)!: c'est moi", ChangeType::Custom("mytype"))]
fn retain_type(#[case] message: &str, #[case] expected_type: ChangeType) {
    let actual_type = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .type_;

    assert_eq!(expected_type, actual_type)
}

#[rstest]
#[case("feat(hello): coucou", Some("hello"))]
#[case("fix(world)!: c'est moi", Some("world"))]
#[case("feat: Hello world!", None)]
fn retain_scope(#[case] message: &str, #[case] expected_scope: Option<&str>) {
    let actual_scope = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .scope;

    assert_eq!(expected_scope, actual_scope)
}

#[rstest]
#[case("fix: coucou", "coucou")]
#[case("fix(withscope): c'est moi", "c'est moi")]
#[case("feat:   this is it!   \n\noops", "this is it!")]
fn retain_description(#[case] message: &str, #[case] expected_description: &str) {
    let actual_description = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .description;

    assert_eq!(expected_description, actual_description)
}

#[rstest]
#[case("feat(hello): a description", None)]
#[case("feat!: coucou\n\nHello world!", Some("Hello world!"))]
#[case(
    "fix: desc\n\nA nice \nbody\n\nwith mutliple lines",
    Some("A nice \nbody\n\nwith mutliple lines")
)]
#[case(
    "fix(world)!: ...\n\nA nice \nbody\n\nwith mutliple lines\n\nbefore: footer",
    Some("A nice \nbody\n\nwith mutliple lines")
)]
fn retain_body(#[case] message: &str, #[case] expected_body: Option<&str>) {
    let actual_body = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .body;

    assert_eq!(expected_body, actual_body)
}

#[rstest]
#[case("feat: hello", BreakingInfo::NotBreaking)]
#[case("feat!: hello", BreakingInfo::Breaking)]
#[case(
    "feat: hello\n\nBREAKING CHANGE: Because I had to...",
    BreakingInfo::BreakingWithDescription("Because I had to...")
)]
#[case(
    "feat: hello\n\nwith a body\n\nBREAKING CHANGE #\nThis\n\nis\nlife...",
    BreakingInfo::BreakingWithDescription("This\n\nis\nlife...")
)]
fn retain_breaking_change_description(#[case] message: &str, #[case] expected: BreakingInfo) {
    let actual = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .breaking;

    assert_eq!(expected, actual)
}
