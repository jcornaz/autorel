#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]

use std::error::Error;

use semver::Version;

use crate::bump::Bump;
use crate::cli::Opts;
use crate::config::Config;
use crate::git::Repository;
use crate::release::Release;

mod action;
mod bump;
mod changelog;
mod cli;
mod cmd;
mod config;
mod git;
mod github;
mod release;

fn main() {
    let options = cli::parse();

    match run(options) {
        Ok(None) => println!("Nothing to release"),
        Ok(Some(Release { version, .. })) => {
            println!("\n\nVersion {} successfully released", version)
        }
        Err(err) => eprintln!("\n\n{}", err),
    }
}

fn run(options: Opts) -> Result<Option<Release<Version>>, Box<dyn Error>> {
    let config: Config = config::read(options.config)?;

    match find_release(&config.tag_prefix)? {
        None => Ok(None),
        Some(release) => {
            perform_release(&config, &release, options.dry_run)?;
            Ok(Some(release))
        }
    }
}

fn perform_release(
    config: &Config,
    release: &Release<Version>,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    let version_str = release.version.to_string();
    let title_suffix = version_str.clone() + if dry_run { " (DRY RUN)" } else { "" };

    println!("Verifying version {}", title_suffix);
    cmd::execute_all(&config.hooks.verify, &version_str, dry_run)?;

    if config.changelog {
        println!("\n\nGenerating changelog {}", title_suffix);
        changelog::generate(&config.tag_prefix, &release, dry_run)?;
    }

    println!("\n\nPreparing version {}", title_suffix);
    cmd::execute_all(&config.hooks.prepare, &version_str, dry_run)?;

    println!("\n\nPublishing version {}", title_suffix);
    cmd::execute_all(&config.hooks.publish, &version_str, dry_run)?;

    if let Some(repo) = &config.github_repo {
        let token = std::env::var("GITHUB_TOKEN")?;
        github::Client::new(repo, &token)?.create_release(
            &config.tag_prefix,
            version_str,
            String::default(),
        )?;
    }

    Ok(())
}

fn find_release(tag_prefix: &str) -> Result<Option<Release<Version>>, git::Error> {
    let repo = Repository::open(".")?;
    let release = match repo.find_latest_release::<Version>("v")? {
        None => Some(Release {
            prev_version: None,
            version: Version::new(0, 1, 0),
        }),
        Some(prev_version) => repo
            .load_changelog(&format!("{}{}", tag_prefix, prev_version))?
            .semver_scope()
            .map(|scope| Release {
                prev_version: Some(prev_version.clone()),
                version: prev_version.bumped(scope),
            }),
    };

    Ok(release)
}
