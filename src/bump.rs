use semver::Version;

use crate::scope::Scope;

pub trait Bump: Sized {
    fn bump(&mut self, scope: Scope);

    #[inline]
    fn bumped(mut self, scope: Scope) -> Self {
        self.bump(scope);
        self
    }
}

impl Bump for Version {
    fn bump(&mut self, scope: Scope) {
        match (self.major, scope) {
            (0, Scope::Feature) | (_, Scope::Fix) => self.increment_patch(),
            (0, Scope::Breaking) | (_, Scope::Feature) => self.increment_minor(),
            (_, Scope::Breaking) => self.increment_major(),
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
        version.bump(Scope::Breaking);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.3.0")]
    #[case("0.1.0", "0.1.1")]
    #[case("0.1.1", "0.1.2")]
    fn feature(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(Scope::Feature);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.2.4")]
    #[case("0.1.2", "0.1.3")]
    fn fix(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(Scope::Fix);
        assert_eq!(version.to_string(), expected_target_version);
    }
}
