#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use semver::Version;

use crate::cvs::{Commit, Repository};
use crate::scope::{CommitScope, ReleaseScope};

mod cli;

mod scope;

mod cvs;

fn main() {
    cli::parse();

    match find_scope() {
        Ok(CommitScope::Internal) => println!("Nothing to release"),
        Ok(CommitScope::Public(_)) => {
            println!("Releasing");
            run_script_if_exists(".release/verify.sh".into());
            run_script_if_exists(".release/prepare.sh".into());
            run_script_if_exists(".release/publish.sh".into());
        }
        Err(err) => eprintln!("{}", err),
    }
}

fn find_scope() -> Result<CommitScope, cvs::Error> {
    let repo = Repository::open(".")?;
    match repo.find_latest_release::<Version>("v")? {
        None => Ok(CommitScope::Public(ReleaseScope::Feature)),
        Some(prev_version) => repo.find_change_scope(&format!("v{}", prev_version)),
    }
}

impl From<cvs::Commit<'_>> for CommitScope {
    fn from(commit: Commit<'_>) -> Self {
        commit
            .message()
            .map(Self::from_commit_message)
            .unwrap_or_default()
    }
}

fn run_script_if_exists(script: PathBuf) -> bool {
    if script.exists() {
        run(script)
    } else {
        true
    }
}

fn run(script: impl AsRef<OsStr>) -> bool {
    match Command::new(script).status() {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}
