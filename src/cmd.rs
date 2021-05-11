use std::fmt::{Display, Formatter};
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::{fmt, io};

#[derive(Debug)]
pub struct Error {
    cmd: String,
    cause: Cause,
}

#[derive(Debug)]
pub enum Cause {
    CannotRunCmd(Option<io::Error>),
    CmdFailed(ExitStatus),
}

impl From<io::Error> for Cause {
    fn from(err: io::Error) -> Self {
        Cause::CannotRunCmd(Some(err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Cause::CannotRunCmd(Some(err)) => write!(f, "Cannot run {}: {}", self.cmd, err),
            Cause::CannotRunCmd(None) => write!(f, "Cannot run {}", self.cmd),
            Cause::CmdFailed(exit_status) => match exit_status.code() {
                None => write!(f, "{} was terminated", self.cmd),
                Some(code) => write!(f, "{} failed (status code: {})", self.cmd, code),
            },
        }
    }
}

impl std::error::Error for Error {}

pub fn execute_all(
    cmds: &[impl AsRef<str>],
    version: impl AsRef<str>,
    dry_run: bool,
) -> Result<(), Error> {
    for cmd in cmds {
        let cmd = cmd.as_ref().replace("{{version}}", version.as_ref());
        println!("> {}", cmd);

        if !dry_run {
            execute(&cmd).map_err(|cause| Error { cmd, cause })?;
        }
    }
    Ok(())
}

fn execute(cmd: &str) -> Result<(), Cause> {
    let mut process = Command::new("sh").stdin(Stdio::piped()).spawn()?;
    match &mut process.stdin {
        None => return Err(Cause::CannotRunCmd(None)),
        Some(stdin) => stdin.write_all(cmd.as_bytes())?,
    }

    let exit_status = process.wait()?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(Cause::CmdFailed(exit_status))
    }
}
