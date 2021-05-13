use semver::Version;

use autorel_core::SemverScope;

pub trait Bump: Sized {
    fn bump(&mut self, scope: SemverScope);

    #[inline]
    fn bumped(mut self, scope: SemverScope) -> Self {
        self.bump(scope);
        self
    }
}

impl Bump for Version {
    fn bump(&mut self, scope: SemverScope) {
        match (self.major, scope) {
            (0, SemverScope::Feature) | (_, SemverScope::Fix) => self.increment_patch(),
            (0, SemverScope::Breaking) | (_, SemverScope::Feature) => self.increment_minor(),
            (_, SemverScope::Breaking) => self.increment_major(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1.2.3", "2.0.0")]
    #[case("2.0.0", "3.0.0")]
    #[case("0.1.2", "0.2.0")]
    #[case("0.2.0", "0.3.0")]
    fn breaking_change(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Breaking);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.3.0")]
    #[case("0.1.0", "0.1.1")]
    #[case("0.1.1", "0.1.2")]
    fn feature(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Feature);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.2.4")]
    #[case("0.1.2", "0.1.3")]
    fn fix(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Fix);
        assert_eq!(version.to_string(), expected_target_version);
    }
}
