#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::error::Error;

use semver::Version;

use crate::bump::Bump;
use crate::cli::Opts;
use crate::config::Config;
use crate::cvs::{Commit, Repository};
use crate::scope::Scope;

mod bump;
mod cli;
mod cmd;
mod config;
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

fn run(options: Opts) -> Result<Option<Version>, Box<dyn Error>> {
    let config: Config = config::read(options.config)?;

    match find_next_version()? {
        None => Ok(None),
        Some(version) => {
            println!("Verifying version {}", version);
            cmd::execute_all(&config.hooks.verify, &version)?;
            println!("Preparing version {}", version);
            cmd::execute_all(&config.hooks.prepare, &version)?;
            println!("Publishing version {}", version);
            cmd::execute_all(&config.hooks.publish, &version)?;
            Ok(Some(version))
        }
    }
}

fn find_next_version() -> Result<Option<Version>, cvs::Error> {
    let repo = Repository::open(".")?;
    let version = match repo.find_latest_release::<Version>("v")? {
        None => Some(Version::new(0, 1, 0)),
        Some(prev_version) => repo
            .find_change_scope::<Option<Scope>>(&format!("v{}", prev_version))?
            .map(|scope| prev_version.bumped(scope)),
    };

    Ok(version)
}

impl From<cvs::Commit<'_>> for Option<Scope> {
    fn from(commit: Commit<'_>) -> Self {
        commit.message().and_then(scope::parse_commit_message)
    }
}
