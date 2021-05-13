use rstest::rstest;

use autorel_core::{Change, ChangeType, ChangeTypeWithDesc};

#[test]
fn can_parse_empty_commit_message() {
    assert!(Change::parse_conventional_commit("").is_none())
}

#[rstest]
#[case("")]
#[case("Hello world")]
#[case("Hello world\n\nwith multiple lines")]
fn non_conventional_commit_is_internal(#[case] message: &str) {
    assert_eq!(None, Change::parse_conventional_commit(message));
}

#[rstest]
#[case("feat!: Hello world")]
#[case("feat(withscope)!: Hello world")]
#[case("fix!: Hello world")]
#[case("other(withscope)!: Hello world")]
#[case("feat: Hello world\n\nBREAKING CHANGE: This is breaking")]
#[case("other: Hello world\n\nBREAKING CHANGE: This is breaking")]
fn recognize_breaking_changes(#[case] message: &str) {
    let type_ = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .type_
        .without_description();
    assert!(matches!(type_, ChangeType::Breaking));
}

#[rstest]
#[case("feat: Hello world")]
#[case("feat: a\n\n")]
#[case("feat(withscope): Hello world")]
#[case("feat: Hello world\n\nwith multiple lines")]
fn recognize_feature(#[case] message: &str) {
    let type_ = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .type_
        .without_description();
    assert!(matches!(type_, ChangeType::Feature));
}

#[rstest]
#[case("fix: Hello world")]
#[case("fix(withscope): Hello world")]
#[case("fix: Hello world\n\nwith multiple lines")]
fn recognize_fix(#[case] message: &str) {
    let type_ = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .type_
        .without_description();
    assert!(matches!(type_, ChangeType::Fix));
}

#[rstest]
#[case("chore: Hello world")]
#[case("chore: Hello world!")]
#[case("chore: Hello !: world")]
#[case("featuring:")]
#[case("tests(withscope): Hello world")]
#[case("tests(with!scope): Hello world")]
#[case("refactor: Hello world\n\nwith multiple lines")]
fn recognize_internal_changes(#[case] message: &str) {
    assert_eq!(None, Change::parse_conventional_commit(message));
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
#[case("feat!: hello", None)]
#[case(
    "feat: hello\n\nBREAKING CHANGE: Because I had to...",
    Some("Because I had to...")
)]
#[case(
    "feat: hello\n\nwith a body\n\nBREAKING CHANGE #\nThis\n\nis\nlife...",
    Some("This\n\nis\nlife...")
)]
fn retain_breaking_change_description(
    #[case] message: &str,
    #[case] expected_description: Option<&str>,
) {
    let type_ = Change::parse_conventional_commit(message)
        .expect("Failed to parse commit")
        .type_;

    let actual_description = match type_ {
        ChangeTypeWithDesc::Breaking(desc) => desc,
        _ => panic!("Was not a breaking change!"),
    };

    assert_eq!(expected_description, actual_description)
}
