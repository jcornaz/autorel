#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{self, Command};

use semver::Version;

use crate::bump::Bump;
use crate::cvs::{Commit, Repository};
use crate::scope::{CommitScope, ReleaseScope};

mod bump;
mod cli;
mod cvs;
mod scope;

fn main() {
    cli::parse();

    match find_next_version() {
        Ok(None) => println!("Nothing to release"),
        Ok(Some(version)) => {
            println!("Releasing {}", version);
            run_script_if_exists(".release/verify.sh".into(), &version);
            run_script_if_exists(".release/prepare.sh".into(), &version);
            run_script_if_exists(".release/publish.sh".into(), &version);
        }
        Err(err) => eprintln!("{}", err),
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

fn run_script_if_exists(script: PathBuf, version: &Version) {
    if script.exists() && !run(script, &version) {
        eprintln!("A release script failed. Aborting.");
        process::exit(1)
    }
}

fn run(script: impl AsRef<OsStr>, version: &Version) -> bool {
    match Command::new(script).arg(version.to_string()).status() {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}
