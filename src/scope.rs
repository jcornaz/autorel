#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scope {
    Fix,
    Feature,
    Breaking,
}

pub fn parse_commit_message(message: &str) -> Option<Scope> {
    if message.contains("BREAKING CHANGE:") {
        return Some(Scope::Breaking);
    }

    match message.splitn(2, ':').next() {
        Some(type_and_scope) => parse_type_and_scope(type_and_scope),
        None => None,
    }
}

fn parse_type_and_scope(type_and_scope: &str) -> Option<Scope> {
    if type_and_scope.ends_with('!') {
        return Some(Scope::Breaking);
    }

    let commit_type = type_and_scope
        .splitn(2, '(')
        .next()
        .unwrap_or(type_and_scope);

    match commit_type {
        "feat" => Some(Scope::Feature),
        "fix" => Some(Scope::Fix),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn can_parse_empty_commit_message() {
        assert!(parse_commit_message("").is_none())
    }

    #[test]
    fn ord_is_from_smallest_to_biggest_scope() {
        let expected = vec![
            None,
            Some(Scope::Fix),
            Some(Scope::Feature),
            Some(Scope::Breaking),
        ];

        let mut actual = expected.clone();
        actual.sort();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("")]
    #[case("Hello world")]
    #[case("Hello world\n\nwith multiple lines")]
    fn non_conventional_commit_is_internal(#[case] message: &str) {
        assert_eq!(None, parse_commit_message(message));
    }

    #[rstest]
    #[case("feat!: Hello world")]
    #[case("feat(withscope)!: Hello world")]
    #[case("fix!: Hello world")]
    #[case("other(withscope)!: Hello world")]
    #[case("feat: Hello world\n\nBREAKING CHANGE: This is breaking")]
    #[case("other: Hello world\n\nBREAKING CHANGE: This is breaking")]
    fn recognize_breaking_changes(#[case] message: &str) {
        assert_eq!(Some(Scope::Breaking), parse_commit_message(message));
    }

    #[rstest]
    #[case("feat: Hello world")]
    #[case("feat: a\n\n")]
    #[case("feat(withscope): Hello world")]
    #[case("feat: Hello world\n\nwith multiple lines")]
    fn recognize_feature(#[case] message: &str) {
        assert_eq!(Some(Scope::Feature), parse_commit_message(message));
    }

    #[rstest]
    #[case("fix: Hello world")]
    #[case("fix(withscope): Hello world")]
    #[case("fix: Hello world\n\nwith multiple lines")]
    fn recognize_fix(#[case] message: &str) {
        assert_eq!(Some(Scope::Fix), parse_commit_message(message));
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
        assert_eq!(None, parse_commit_message(message));
    }
}
