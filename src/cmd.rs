use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::{fmt, io};

use semver::Version;

#[derive(Debug)]
pub struct Error {
    cmd: OsString,
    cause: Cause,
}

#[derive(Debug)]
pub enum Cause {
    CannotRunCmd(io::Error),
    CmdFailed(ExitStatus),
}

impl From<io::Error> for Cause {
    fn from(err: io::Error) -> Self {
        Cause::CannotRunCmd(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Cause::CannotRunCmd(err) => write!(f, "Cannot run {:?}: {}", self.cmd, err),
            Cause::CmdFailed(exit_status) => match exit_status.code() {
                None => write!(f, "{:?} was terminated", self.cmd),
                Some(code) => write!(f, "{:?} failed (status code: {})", self.cmd, code),
            },
        }
    }
}

impl std::error::Error for Error {}

pub fn run_script_if_exists(script: PathBuf, version: &Version) -> Result<(), Error> {
    if script.exists() {
        run_script(&script, &version).map_err(|cause| Error {
            cmd: script.into(),
            cause,
        })
    } else {
        Ok(())
    }
}

fn run_script(script: impl AsRef<OsStr>, version: &Version) -> Result<(), Cause> {
    let status = Command::new(script).arg(version.to_string()).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Cause::CmdFailed(status))
    }
}
