use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;

lazy_static! {
    static ref TOKEN_REGEX: Regex = Regex::new("^\\w+$").unwrap();
    static ref REPO_REGEX: Regex = Regex::new("^[0-9a-zA-Z-_\\.]+/[0-9a-zA-Z-_\\.]+$").unwrap();
}

pub struct Client {
    client: reqwest::blocking::Client,
    release_endpoint: String,
}

impl Client {
    pub fn new(repo: &str, token: &str) -> Result<Self, Error> {
        if !REPO_REGEX.is_match(repo) {
            return Err(Error::InvalidRepo(repo.to_string()));
        }
        if !TOKEN_REGEX.is_match(token) {
            return Err(Error::InvalidToken);
        }

        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("token {}", token)).unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .user_agent("autorel")
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            release_endpoint: format!("https://api.github.com/repos/{}/releases", repo),
        })
    }

    pub fn create_release(
        &self,
        tag_prefix: &str,
        version_str: String,
        body: String,
    ) -> Result<(), Error> {
        let mut data: HashMap<&str, String> = HashMap::with_capacity(3);
        data.insert("tag_name", format!("{}{}", tag_prefix, version_str));
        data.insert("name", version_str);
        data.insert("body", body);

        let status = self
            .client
            .post(&self.release_endpoint)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()?
            .status();

        if !status.is_success() {
            return Err(Error::ApiError(status));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CannotReachApi(reqwest::Error),
    ApiError(StatusCode),
    InvalidToken,
    InvalidRepo(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotReachApi(err) => err.fmt(f),
            Error::ApiError(code) => write!(f, "Github responded: {}", code),
            Error::InvalidToken => write!(f, "Invalid github token"),
            Error::InvalidRepo(repo) => write!(f, "Not a valid github repository: \"{}\"", repo),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::CannotReachApi(err)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("user/repo", "1234567890abcdefg_z")]
    #[case("jcornaz/autorel", "1234567890abcdefg_z")]
    #[case("jcornaz/heron", "1234567890abcdefg_z")]
    #[case("a-b/c-d", "1234567890abcdefg_z")]
    #[case("a_b/c_d", "1234567890abcdefg_z")]
    #[case("a.b/c.d", "1234567890abcdefg_z")]
    fn successful_new_client(#[case] repo: &str, #[case] fake_token: &str) {
        assert!(Client::new(repo, fake_token).is_ok())
    }

    #[rstest]
    #[case("user")]
    #[case("user/repo/")]
    #[case("a/b?")]
    #[case("a/b/c")]
    fn invalid_repo(#[case] repo: &str) {
        match Client::new(repo, "deadbeef") {
            Ok(_) => panic!("Client creation should've failed"),
            Err(Error::InvalidRepo(actual)) => assert_eq!(repo, actual),
            Err(err) => panic!("Client creation failed with incorrect error: {}", err),
        }
    }

    #[rstest]
    #[case("hello world")]
    #[case("hello world!")]
    #[case("a$#")]
    fn invalid_token(#[case] token: &str) {
        match Client::new("user/repo", token) {
            Ok(_) => panic!("Client creation should've failed"),
            Err(Error::InvalidToken) => (),
            Err(err) => panic!("Client creation failed with incorrect error: {}", err),
        }
    }
}
