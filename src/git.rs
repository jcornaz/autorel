use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::{fmt, io};

use git2::{ObjectType, Oid, Repository, Signature};

use crate::config::CommitConfig;

#[derive(Debug)]
pub enum Error {
    LibGitErr(git2::Error),
    IoError(io::Error),
    StatusCode(ExitStatus),
    RepositoryDirtyAfterCommit,
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
            Error::RepositoryDirtyAfterCommit => {
                writeln!(f, "Git repository is still dirty after committing files.")
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn check_is_clean(_: &Repository) -> Result<(), Error> {
    println!("> check if repository is clean");
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;

    if !output.status.success() {
        Err(Error::StatusCode(output.status))
    } else if output.stdout.is_empty() {
        Ok(())
    } else {
        Err(Error::RepositoryDirtyAfterCommit)
    }
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
    tag_prefix: &str,
    version_str: &str,
    dry_run: bool,
) -> Result<(), Error> {
    let signature = match repo.signature() {
        Ok(signature) => signature,
        Err(_) => Signature::now("autorel", "autorel")?,
    };

    let tree = stage_files(repo, &config.files, dry_run)?;

    let commit = perform_commit(
        repo,
        &signature,
        tree,
        &config.message,
        version_str,
        dry_run,
    )?;

    check_is_clean(&repo)?;

    tag(
        &repo,
        &signature,
        commit,
        &tag_prefix,
        &version_str,
        dry_run,
    )?;

    push(&repo, dry_run)
}

pub fn tag(
    repo: &Repository,
    signature: &Signature<'_>,
    last_commit_id: Oid,
    prefix: &str,
    version: &str,
    dry_run: bool,
) -> Result<(), Error> {
    let tag_name = format!("{}{}", prefix, version);
    println!("> git tag {}", tag_name);

    let object = repo.find_object(last_commit_id, Some(ObjectType::Commit))?;

    if !dry_run {
        repo.tag(
            &tag_name,
            &object,
            &signature,
            &format!("Release {}", version),
            false,
        )
        .map(|_| ())
        .map_err(Error::from)
    } else {
        Ok(())
    }
}

pub fn push(_: &Repository, dry_run: bool) -> Result<(), Error> {
    println!("> git push");

    if !dry_run {
        let status = Command::new("git")
            .arg("push")
            .arg("--follow-tags")
            .status()?;

        if !status.success() {
            return Err(Error::from(status));
        }
    }

    Ok(())
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
    signature: &Signature<'_>,
    tree_id: Oid,
    commit_message: &str,
    version_str: &str,
    dry_run: bool,
) -> Result<Oid, git2::Error> {
    let commit_message = commit_message.replace("{{version}}", version_str);
    println!("> git commit -m \"{}\"", commit_message);

    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    let last_commit_id = walker.next().expect("No previous commit found")?;
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
    } else {
        Ok(last_commit_id)
    }
}
