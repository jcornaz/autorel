#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::error::Error;

use semver::Version;

use crate::bump::Bump;
use crate::cli::Opts;
use crate::cvs::{Commit, Repository};
use crate::scope::{CommitScope, ReleaseScope};

mod bump;
mod cli;
mod cmd;
mod cvs;
mod scope;

fn main() {
    let options = cli::parse();

    match run(options) {
        Ok(None) => println!("Nothing to release"),
        Ok(Some(version)) => println!("Version {} released", version),
        Err(err) => eprintln!("{}", err),
    }
}

fn run(_: Opts) -> Result<Option<Version>, Box<dyn Error>> {
    match find_next_version()? {
        None => Ok(None),
        Some(version) => {
            println!("Releasing version {}", version);
            cmd::run_script_if_exists(".release/verify.sh".into(), &version)?;
            cmd::run_script_if_exists(".release/prepare.sh".into(), &version)?;
            cmd::run_script_if_exists(".release/publish.sh".into(), &version)?;
            Ok(Some(version))
        }
    }
}

fn find_next_version() -> Result<Option<Version>, cvs::Error> {
    let repo = Repository::open(".")?;
    let version = match repo.find_latest_release::<Version>("v")? {
        None => Some(Version::new(0, 1, 0)),
        Some(prev_version) => repo
            .find_change_scope::<Option<ReleaseScope>>(&format!("v{}", prev_version))?
            .map(|scope| prev_version.bumped(scope)),
    };

    Ok(version)
}

impl From<cvs::Commit<'_>> for CommitScope {
    fn from(commit: Commit<'_>) -> Self {
        commit
            .message()
            .map(Self::from_commit_message)
            .unwrap_or_default()
    }
}

impl From<cvs::Commit<'_>> for Option<ReleaseScope> {
    fn from(commit: Commit<'_>) -> Self {
        match CommitScope::from(commit) {
            CommitScope::Internal => None,
            CommitScope::Public(scope) => Some(scope),
        }
    }
}
