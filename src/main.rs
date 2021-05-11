#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::error::Error;

use semver::Version;

use crate::bump::Bump;
use crate::cli::Opts;
use crate::config::Config;
use crate::cvs::{Commit, Repository};
use crate::release::Release;
use crate::scope::Scope;

mod bump;
mod changelog;
mod cli;
mod cmd;
mod config;
mod cvs;
mod release;
mod scope;

fn main() {
    let options = cli::parse();

    match run(options) {
        Ok(None) => println!("Nothing to release"),
        Ok(Some(Release { version, .. })) => println!("Version {} released", version),
        Err(err) => eprintln!("{}", err),
    }
}

fn run(options: Opts) -> Result<Option<Release<Version>>, Box<dyn Error>> {
    let config: Config = config::read(options.config)?;

    match find_release(&config.tag_prefix)? {
        None => Ok(None),
        Some(release) => {
            perform_release(config, &release)?;
            Ok(Some(release))
        }
    }
}

fn perform_release(config: Config, release: &Release<Version>) -> Result<(), Box<dyn Error>> {
    let version_str = release.version.to_string();

    println!("Verifying version {}", version_str);
    cmd::execute_all(&config.hooks.verify, &version_str)?;

    if config.changelog {
        println!("Generating changelog {}", version_str);
        changelog::generate(&config.tag_prefix, &release)?;
    }

    println!("Preparing version {}", version_str);
    cmd::execute_all(&config.hooks.prepare, &version_str)?;

    println!("Publishing version {}", version_str);
    cmd::execute_all(&config.hooks.publish, &version_str)?;

    Ok(())
}

fn find_release(tag_prefix: &str) -> Result<Option<Release<Version>>, cvs::Error> {
    let repo = Repository::open(".")?;
    let release = match repo.find_latest_release::<Version>("v")? {
        None => Some(Release {
            prev_version: None,
            version: Version::new(0, 1, 0),
        }),
        Some(prev_version) => repo
            .find_change_scope::<Option<Scope>>(&format!("{}{}", tag_prefix, prev_version))?
            .map(|scope| Release {
                prev_version: Some(prev_version.clone()),
                version: prev_version.bumped(scope),
            }),
    };

    Ok(release)
}

impl From<cvs::Commit<'_>> for Option<Scope> {
    fn from(commit: Commit<'_>) -> Self {
        commit.message().and_then(scope::parse_commit_message)
    }
}
