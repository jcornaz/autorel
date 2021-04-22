use conventional_commits_parser::Commit;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scope {
    Fix,
    Feature,
    Breaking,
}

pub fn parse_commit_message(message: &str) -> Option<Scope> {
    conventional_commits_parser::parse_commit_msg(message)
        .map(from_commit)
        .unwrap_or_default()
}

fn from_commit(commit: Commit<'_>) -> Option<Scope> {
    if commit.is_breaking_change {
        Some(Scope::Breaking)
    } else {
        match commit.ty {
            "feat" => Some(Scope::Feature),
            "fix" => Some(Scope::Fix),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

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
    #[case("tests(withscope): Hello world")]
    #[case("refactor: Hello world\n\nwith multiple lines")]
    fn recognize_internal_changes(#[case] message: &str) {
        assert_eq!(None, parse_commit_message(message));
    }
}
