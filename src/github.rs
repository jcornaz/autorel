use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde_derive::Deserialize;
use url::form_urlencoded;

use autorel_chlg::ChangeLog;

use crate::config::GithubConfig;

lazy_static! {
    static ref TOKEN_REGEX: Regex = Regex::new("^\\w+$").unwrap();
    static ref REPO_REGEX: Regex = Regex::new("^[0-9a-zA-Z-_\\.]+/[0-9a-zA-Z-_\\.]+$").unwrap();
}

pub fn create_github_release(
    config: &GithubConfig,
    tag_prefix: &str,
    version_str: String,
    changelog: &ChangeLog,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("GITHUB_TOKEN").map_err(|_| Error::InvalidToken)?;
    let client = Client::new(&config.repo, &token)?;

    println!("> Create release {}", version_str);
    let upload_url = if !dry_run {
        client.create_release(tag_prefix, version_str, changelog.markdown().to_string())?
    } else {
        String::default()
    };

    for file in &config.files {
        println!("> Upload {}", file.display());
        if !dry_run {
            client.upload_file(&upload_url, &file)?;
        }
    }

    Ok(())
}

struct Client {
    client: reqwest::blocking::Client,
    release_endpoint: String,
}

impl Client {
    fn new(repo: &str, token: &str) -> Result<Self, Error> {
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
        headers.insert(
            "Accept",
            HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
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

    fn create_release(
        &self,
        tag_prefix: &str,
        version_str: String,
        body: String,
    ) -> Result<String, Error> {
        let mut data: HashMap<&str, String> = HashMap::with_capacity(3);
        data.insert("tag_name", format!("{}{}", tag_prefix, version_str));
        data.insert("name", version_str);
        data.insert("body", body);

        let response = self
            .client
            .post(&self.release_endpoint)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()?;

        if !response.status().is_success() {
            return Err(Error::ApiError(response.status()));
        }

        let payload: ReleasePayload = response.json()?;

        Ok(payload.upload_url)
    }

    fn upload_file(&self, url: &str, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = file.display().to_string();
        let parameters = form_urlencoded::Serializer::new(String::new())
            .append_pair("name", &file_name)
            .append_pair("label", &file_name)
            .finish();

        let url = format!("{}?{}", url.splitn(2, '{').next().unwrap(), parameters);

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/octet-stream")
            .body(File::open(file)?)
            .send()?;

        if !response.status().is_success() {
            eprintln!("{:?}", response);
            Err(Box::new(Error::ApiError(response.status())))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReleasePayload {
    upload_url: String,
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
            Error::InvalidToken => write!(f, "Github token (in `GITHUB_TOKEN` env. variable) is absent, invalid or doesn't allow to create a release."),
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
