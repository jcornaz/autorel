#![allow(clippy::upper_case_acronyms)]
#[macro_use]
extern crate pest_derive;

pub use changelog::{ChangeLog, Section};

mod changelog;
mod conventional_commit_parser;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Change<'a> {
    pub type_: ChangeType<'a>,
    pub scope: Option<&'a str>,
    pub breaking: BreakingInfo<'a>,
    pub description: &'a str,
    pub body: Option<&'a str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BreakingInfo<'a> {
    NotBreaking,
    Breaking,
    BreakingWithDescription(&'a str),
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

    pub fn semver_scope(&self) -> Option<SemverScope> {
        match (self.breaking, self.type_) {
            (BreakingInfo::NotBreaking, ChangeType::Feature) => Some(SemverScope::Feature),
            (BreakingInfo::NotBreaking, ChangeType::Fix) => Some(SemverScope::Fix),
            (BreakingInfo::NotBreaking, ChangeType::Custom(_)) => None,
            (BreakingInfo::Breaking, _) | (BreakingInfo::BreakingWithDescription(_), _) => {
                Some(SemverScope::Breaking)
            }
        }
    }

    pub fn is_breaking(&self) -> bool {
        match self.breaking {
            BreakingInfo::NotBreaking => false,
            BreakingInfo::Breaking => true,
            BreakingInfo::BreakingWithDescription(_) => false,
        }
    }
}
