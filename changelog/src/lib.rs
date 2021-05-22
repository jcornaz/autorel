#![allow(clippy::upper_case_acronyms)]
#[macro_use]
extern crate pest_derive;

pub use changelog::ChangeLog;

mod changelog;
mod conventional_commit_parser;
pub mod git;
pub mod markdown;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Change<'a> {
    pub type_: ChangeType<'a>,
    pub scope: Option<&'a str>,
    pub breaking: BreakingInfo<'a>,
    pub description: &'a str,
    pub body: Option<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BreakingInfo<'a> {
    NotBreaking,
    Breaking,
    BreakingWithDescriptions(Vec<&'a str>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeType<'a> {
    Fix,
    Feature,
    Custom(&'a str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SemverScope {
    Fix,
    Feature,
    Breaking,
}

impl<'a> Change<'a> {
    #[inline]
    pub fn new(type_: ChangeType<'a>, description: &'a str) -> Self {
        Self {
            type_,
            scope: None,
            breaking: BreakingInfo::NotBreaking,
            description,
            body: None,
        }
    }

    #[inline]
    pub fn parse_conventional_commit(message: &'a str) -> Option<Self> {
        conventional_commit_parser::parse(message)
    }
}
