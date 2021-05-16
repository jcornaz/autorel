use std::fmt::{Display, Formatter};
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    CannotRunCmd(Option<io::Error>),
    CmdFailed(ExitStatus),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::CannotRunCmd(Some(err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::CannotRunCmd(Some(err)) => write!(f, "Cannot invoke shell: {}", err),
            Error::CannotRunCmd(None) => write!(f, "Cannot invoke shell"),
            Error::CmdFailed(exit_status) => match exit_status.code() {
                None => write!(f, "A command returned failed"),
                Some(code) => write!(f, "A command returned failed (status code: {})", code),
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
    let mut shell = Command::new("sh").stdin(Stdio::piped()).spawn()?;

    let mut stdin = shell.stdin.take().ok_or(Error::CannotRunCmd(None))?;

    for cmd in cmds {
        let mut cmd = cmd.as_ref().replace("{{version}}", version.as_ref());

        stdin.write_all(format!("echo '> {}'\n", cmd.replace('\'', "\\'")).as_bytes())?;

        if !dry_run {
            cmd.push('\n');
            stdin.write_all(cmd.as_bytes())?;
        }
    }

    drop(stdin);

    let status = shell.wait()?;

    if !status.success() {
        Err(Error::CmdFailed(status))
    } else {
        Ok(())
    }
}
