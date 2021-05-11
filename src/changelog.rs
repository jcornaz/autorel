use std::fmt;
use std::fmt::{Display, Formatter};

use clog::Clog;
use semver::Version;

use crate::release::Release;

#[derive(Debug)]
pub struct Error(clog::error::Error);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<clog::error::Error> for Error {
    fn from(err: clog::error::Error) -> Self {
        Self(err)
    }
}

pub fn generate(tag_prefix: &str, release: &Release<Version>, dry_run: bool) -> Result<(), Error> {
    Clog::new()
        .and_then(|mut clog| {
            clog.version(release.version.to_string())
                .patch_ver(release.version.is_prerelease() || release.version.patch > 0);

            if let Some(prev_version) = &release.prev_version {
                clog.from(format!("{}{}", tag_prefix, prev_version));
            }

            if dry_run {
                clog.outfile = None
            } else if clog.outfile.is_none() {
                clog.outfile = Some(String::from("CHANGELOG.md"));
            }

            clog.write_changelog()
        })
        .map_err(Error)
}
