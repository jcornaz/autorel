use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::Utc;
use semver::Version;

use crate::release::Release;

#[derive(Debug)]
pub struct Error(io::Error);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self(err)
    }
}

pub fn generate(release: &Release<Version>, dry_run: bool) -> Result<(), Error> {
    let date = Utc::now().format("%Y-%m-%d");

    let file: PathBuf = PathBuf::from("CHANGELOG.md");

    let mut changelog = format!(
        "## {version} - {date}\n\n{changes}",
        version = release.version,
        date = date,
        changes = release.changelog.markdown(),
    );
    println!("{}", changelog.trim());

    if !dry_run {
        if file.exists() {
            changelog.push('\n');
            changelog.push_str(&fs::read_to_string(&file)?);
        }

        fs::write(file, changelog)?;
    }

    Ok(())
}
