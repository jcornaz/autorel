use conventional_commits_parser::Commit;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReleaseScope {
    Fix,
    Feature,
    Breaking,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CommitScope {
    Internal,
    Public(ReleaseScope),
}

impl Default for CommitScope {
    fn default() -> Self {
        Self::Internal
    }
}

impl CommitScope {
    pub fn from_commit_message(message: &str) -> CommitScope {
        conventional_commits_parser::parse_commit_msg(message)
            .map(CommitScope::from)
            .unwrap_or_default()
    }
}

impl<'a> From<conventional_commits_parser::Commit<'a>> for CommitScope {
    fn from(commit: Commit<'a>) -> Self {
        if commit.is_breaking_change {
            Self::Public(ReleaseScope::Breaking)
        } else {
            match commit.ty {
                "feat" => Self::Public(ReleaseScope::Feature),
                "fix" => Self::Public(ReleaseScope::Fix),
                _ => Self::Internal,
            }
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
            CommitScope::Internal,
            CommitScope::Public(ReleaseScope::Fix),
            CommitScope::Public(ReleaseScope::Feature),
            CommitScope::Public(ReleaseScope::Breaking),
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
        assert_eq!(
            CommitScope::Internal,
            CommitScope::from_commit_message(message)
        );
    }

    #[rstest]
    #[case("feat!: Hello world")]
    #[case("feat(withscope)!: Hello world")]
    #[case("fix!: Hello world")]
    #[case("other(withscope)!: Hello world")]
    #[case("feat: Hello world\n\nBREAKING CHANGE: This is breaking")]
    #[case("other: Hello world\n\nBREAKING CHANGE: This is breaking")]
    fn recognize_breaking_changes(#[case] message: &str) {
        assert_eq!(
            CommitScope::Public(ReleaseScope::Breaking),
            CommitScope::from_commit_message(message)
        );
    }

    #[rstest]
    #[case("feat: Hello world")]
    #[case("feat(withscope): Hello world")]
    #[case("feat: Hello world\n\nwith multiple lines")]
    fn recognize_feature(#[case] message: &str) {
        assert_eq!(
            CommitScope::Public(ReleaseScope::Feature),
            CommitScope::from_commit_message(message)
        );
    }

    #[rstest]
    #[case("fix: Hello world")]
    #[case("fix(withscope): Hello world")]
    #[case("fix: Hello world\n\nwith multiple lines")]
    fn recognize_fix(#[case] message: &str) {
        assert_eq!(
            CommitScope::Public(ReleaseScope::Fix),
            CommitScope::from_commit_message(message)
        );
    }

    #[rstest]
    #[case("chore: Hello world")]
    #[case("tests(withscope): Hello world")]
    #[case("refactor: Hello world\n\nwith multiple lines")]
    fn recognize_internal_changes(#[case] message: &str) {
        assert_eq!(
            CommitScope::Internal,
            CommitScope::from_commit_message(message)
        );
    }
}
