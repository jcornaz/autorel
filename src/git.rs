use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::{fmt, io};

use git2::{ObjectType, Oid, Repository};

use crate::config::CommitConfig;

#[derive(Debug)]
pub enum Error {
    LibGitErr(git2::Error),
    IoError(io::Error),
    StatusCode(ExitStatus),
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Self::LibGitErr(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ExitStatus> for Error {
    fn from(err: ExitStatus) -> Self {
        Self::StatusCode(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::LibGitErr(err) => err.fmt(f),
            Error::IoError(err) => write!(f, "Failed to invoke git: {}", err),
            Error::StatusCode(status) => {
                write!(f, "Git command failed.")?;
                if let Some(code) = status.code() {
                    write!(f, " Status code: {}", code)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn is_clean(_: &Repository) -> Result<bool, Error> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;

    if !output.status.success() {
        return Err(Error::StatusCode(output.status));
    }
    Ok(output.stdout.is_empty())
}

pub fn find_latest_release<V: FromStr + Ord>(
    repo: &Repository,
    tag_prefix: &str,
) -> Result<Option<V>, Error> {
    Ok(repo
        .tag_names(Some(&(String::from(tag_prefix) + "*")))?
        .iter()
        .filter_map(|tag| {
            tag.and_then(|it| it.strip_prefix(tag_prefix))
                .and_then(|it| it.parse().ok())
        })
        .max())
}

pub fn commit(
    repo: &Repository,
    config: &CommitConfig,
    version_str: &str,
    dry_run: bool,
) -> Result<(), Error> {
    let oid = stage_files(repo, &config.files, dry_run)?;
    perform_commit(repo, oid, &config.message, version_str, dry_run).map_err(Error::from)
}

pub fn tag(repo: &Repository, name: &str, message: &str) -> Result<(), Error> {
    let signature = repo.signature()?;
    let last_commit_id = find_last_commit_id(repo)?;
    let object = repo.find_object(last_commit_id, Some(ObjectType::Commit))?;

    repo.tag(name, &object, &signature, message, false)
        .map(|_| ())
        .map_err(Error::from)
}

pub fn push(_: &Repository) -> Result<(), Error> {
    let status = Command::new("git")
        .arg("push")
        .arg("--follow-tags")
        .status()?;
    if !status.success() {
        Err(Error::from(status))
    } else {
        Ok(())
    }
}

fn stage_files(repo: &Repository, files: &[PathBuf], dry_run: bool) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    for file in files {
        println!("> git add \"{}\"", file.display());
        if !dry_run {
            index.add_path(&file)?;
        }
    }

    if !dry_run {
        index.write()?;
        index.write_tree()
    } else {
        Ok(Oid::zero())
    }
}

fn perform_commit(
    repo: &Repository,
    tree_id: Oid,
    commit_message: &str,
    version_str: &str,
    dry_run: bool,
) -> Result<(), git2::Error> {
    let commit_message = commit_message.replace("{version}", version_str);
    println!("> git commit -m \"{}\"", commit_message);

    let signature = repo.signature()?;
    let last_commit_id = find_last_commit_id(repo)?;
    let last_commit = repo.find_commit(last_commit_id)?;

    if !dry_run {
        let tree = repo.find_tree(tree_id)?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            &[&last_commit],
        )
        .map(|_| ())
    } else {
        Ok(())
    }
}

fn find_last_commit_id(repo: &Repository) -> Result<Oid, git2::Error> {
    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    walker.next().expect("No previous commit found")
}
