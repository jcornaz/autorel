use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use semver::{Prerelease, Version};
use serde_derive::Deserialize;

use autorel_chlg::SemverScope;

pub trait Bump: Sized {
    fn stabilize(&mut self);
    fn bump(&mut self, scope: SemverScope, pre_release: Option<PreReleaseLabel>);

    #[inline]
    fn bumped(mut self, scope: SemverScope, pre_release: Option<PreReleaseLabel>) -> Self {
        self.bump(scope, pre_release);
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(try_from = "String")]
pub struct PreReleaseLabel(Prerelease);

#[derive(Debug)]
pub enum InvalidPreReleaseLabel {
    Empty,
    SemverIncompatible(semver::Error),
}

impl Bump for Version {
    fn stabilize(&mut self) {
        if self.major < 1 {
            self.major = 1;
            self.minor = 0;
            self.patch = 0;
        }
    }

    fn bump(&mut self, scope: SemverScope, pre_release: Option<PreReleaseLabel>) {
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

        if let Some(PreReleaseLabel(pre)) = pre_release {
            self.pre = format!("{}.1", pre).parse().unwrap();
        }
    }
}

impl FromStr for PreReleaseLabel {
    type Err = InvalidPreReleaseLabel;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pre_release = Prerelease::new(s).map_err(InvalidPreReleaseLabel::SemverIncompatible)?;

        if pre_release.is_empty() {
            Err(InvalidPreReleaseLabel::Empty)
        } else {
            Ok(Self(pre_release))
        }
    }
}

impl TryFrom<String> for PreReleaseLabel {
    type Error = InvalidPreReleaseLabel;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for InvalidPreReleaseLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidPreReleaseLabel::Empty => write!(f, "Empty pre-release label"),
            InvalidPreReleaseLabel::SemverIncompatible(err) => {
                write!(f, "Invalid pre-release label ({})", err)
            }
        }
    }
}

impl std::error::Error for InvalidPreReleaseLabel {}

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
    #[case("0.0.0", "0.1.0")]
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

    #[rstest]
    #[case("0.0.0", SemverScope::Breaking, Some(PreReleaseLabel::from_str("alpha").unwrap()), "0.1.0-alpha.1")]
    #[case("0.1.0", SemverScope::Feature, Some(PreReleaseLabel::from_str("alpha").unwrap()), "0.1.1-alpha.1")]
    #[case("0.1.0", SemverScope::Fix, Some(PreReleaseLabel::from_str("alpha").unwrap()), "0.1.1-alpha.1")]
    // TODO #[case("0.1.1-alpha.1", SemverScope::Fix, Some(PreReleaseLabel::from_str("alpha").unwrap()), "0.1.1-alpha.2")]
    fn pre_release(
        #[case] initial_version: Version,
        #[case] scope: SemverScope,
        #[case] label: Option<PreReleaseLabel>,
        #[case] expected: Version,
    ) {
        let version = initial_version.clone().bumped(scope, label);
        assert_eq!(version, expected)
    }
}
