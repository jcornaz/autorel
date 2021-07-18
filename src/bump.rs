use semver::Version;

use autorel_chlg::SemverScope;

pub trait Bump: Sized {
    fn stabilize(&mut self);
    fn bump(&mut self, scope: SemverScope, pre_release: Option<&str>);

    #[inline]
    fn bumped(mut self, scope: SemverScope, pre_release: Option<&str>) -> Self {
        self.bump(scope, pre_release);
        self
    }
}

impl Bump for Version {
    fn stabilize(&mut self) {
        if self.major < 1 {
            self.major = 1;
            self.minor = 0;
            self.patch = 0;
        }
    }

    fn bump(&mut self, scope: SemverScope, _: Option<&str>) {
        match (self.major, scope) {
            (0, SemverScope::Feature) | (_, SemverScope::Fix) => {
                self.patch += 1;
            }
            (0, SemverScope::Breaking) | (_, SemverScope::Feature) => {
                self.minor += 1;
                self.patch = 0;
            }
            (_, SemverScope::Breaking) => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0.1.0", "1.0.0")]
    #[case("0.1.2", "1.0.0")]
    #[case("0.2.4", "1.0.0")]
    #[case("1.0.0", "1.0.0")]
    #[case("1.2.3", "1.2.3")]
    fn stabilize(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.stabilize();
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "2.0.0")]
    #[case("2.0.0", "3.0.0")]
    #[case("0.1.2", "0.2.0")]
    #[case("0.2.0", "0.3.0")]
    fn breaking_change(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Breaking, None);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.3.0")]
    #[case("0.1.0", "0.1.1")]
    #[case("0.1.1", "0.1.2")]
    fn feature(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Feature, None);
        assert_eq!(version.to_string(), expected_target_version);
    }

    #[rstest]
    #[case("1.2.3", "1.2.4")]
    #[case("0.1.2", "0.1.3")]
    fn fix(#[case] initial_version: &str, #[case] expected_target_version: &str) {
        let mut version: Version = initial_version.parse().unwrap();
        version.bump(SemverScope::Fix, None);
        assert_eq!(version.to_string(), expected_target_version);
    }
}
