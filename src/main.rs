#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![allow(clippy::enum_variant_names)]

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::process;

use git2::Repository;
use semver::Version;

use autorel_chlg::git::ChangeLogRepository;

use crate::bump::Bump;
use crate::cli::Opts;
use crate::config::Config;
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

    match run(&options) {
        Ok(None) => println!("Nothing to release"),
        Ok(Some(Release { version, .. })) => {
            if !options.dry_run {
                println!("\n\nVersion {} successfully released", version)
            }
        }
        Err(err) => {
            eprintln!("\n\n{}", err);
            process::exit(1);
        }
    }
}

fn run(options: &Opts) -> Result<Option<Release<Version>>, Box<dyn Error>> {
    let config: Config = config::read(&options.config)?;

    match find_next_release(&config.tag_prefix)? {
        None => Ok(None),
        Some(mut release) => {
            if release.prev_version.is_none() && !options.force {
                Err(Box::new(PreviousReleaseNotFound))
            } else if !release.changelog.has_feature_or_fix() && !options.force {
                Ok(None)
            } else {
                if options.stable {
                    release.version.stabilize()
                }
                perform_release(&config, &release, options.dry_run)?;
                Ok(Some(release))
            }
        }
    }
}

fn perform_release(
    config: &Config,
    release: &Release<Version>,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    let version_str = release.version.to_string();
    let title_suffix = if dry_run { " [DRY RUN]" } else { "" };
    println!("Releasing version {}{}", version_str, title_suffix);

    if !config.hooks.verify.is_empty() {
        println!("\nVerifying{}", title_suffix);
        cmd::execute_all(&config.hooks.verify, &version_str, dry_run)?;
    }

    if config.changelog {
        println!("\nWriting changelog{}", title_suffix);
        changelog::generate(release, dry_run)?;
    }

    if !config.hooks.prepare.is_empty() {
        println!("\nPreparing{}", title_suffix);
        cmd::execute_all(&config.hooks.prepare, &version_str, dry_run)?;
    }

    println!("\nUpdating git repository{}", title_suffix);
    git::commit(
        &release.repo,
        &config.commit,
        &config.tag_prefix,
        &version_str,
        dry_run,
    )?;

    if !config.hooks.publish.is_empty() {
        println!("\nPublishing{}", title_suffix);
        cmd::execute_all(&config.hooks.publish, &version_str, dry_run)?;
    }

    if let Some(gh_config) = &config.github {
        println!("\nCreate github release{}", title_suffix);
        github::create_github_release(
            gh_config,
            &config.tag_prefix,
            version_str,
            &release.changelog,
            dry_run,
        )?;
    }

    Ok(())
}

fn find_next_release(tag_prefix: &str) -> Result<Option<Release<Version>>, git::Error> {
    let repo = Repository::open(".")?;
    let release = match git::find_latest_release::<Version>(&repo, "v")? {
        None => {
            let changelog = repo.load_changelog(None)?;

            changelog.semver_scope().map(|_| Release {
                prev_version: None,
                version: Version::new(0, 1, 0),
                changelog,
                repo,
            })
        }
        Some(prev_version) => {
            let changelog =
                repo.load_changelog(Some(&format!("{}{}", tag_prefix, prev_version)))?;

            changelog.semver_scope().map(|scope| Release {
                prev_version: Some(prev_version.clone()),
                version: prev_version.bumped(scope),
                changelog,
                repo,
            })
        }
    };

    Ok(release)
}

#[derive(Debug, Copy, Clone)]
struct PreviousReleaseNotFound;

impl Display for PreviousReleaseNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Didn't find any previous version in the tags. Make sure to fetch all history and tags when cloning.")?;
        write!(f, "If this is really the first version to publish, use the `--force` flag to proceed with the release.")
    }
}

impl std::error::Error for PreviousReleaseNotFound {}
