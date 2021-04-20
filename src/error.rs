use std::fmt::{Display, Formatter};
use std::{fmt, io};

use crate::cvs;

#[derive(Debug)]
pub enum Error {
    CannotReadConfigFile(io::Error),
    InvalidConfig(toml::de::Error),
    CvsError(cvs::Error),
    CannotRunCmd(String, io::Error),
    CmdFailure(String),
}

impl From<cvs::Error> for Error {
    fn from(err: cvs::Error) -> Self {
        Self::CvsError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::InvalidConfig(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotReadConfigFile(err) => write!(f, "Cannot read config file: {}", err),
            Error::InvalidConfig(err) => write!(f, "Invalid configuration: {}", err),
            Error::CvsError(err) => write!(f, "Cannot use version control: {}", err),
            Error::CannotRunCmd(cmd, err) => write!(f, "Cannot run command '{}': {}", cmd, err),
            Error::CmdFailure(cmd) => write!(f, "The command '{}' failed", cmd),
        }
    }
}

impl std::error::Error for Error {}