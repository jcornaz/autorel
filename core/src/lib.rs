#![allow(clippy::upper_case_acronyms)]
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Change<'a> {
    pub type_: ChangeType<'a>,
    pub breaking: BreakingInfo<'a>,
    pub scope: Option<&'a str>,
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

#[derive(Parser)]
#[grammar = "conventional_commit.pest"]
struct ConventionalCommitParser;

impl<'a> Change<'a> {
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

    pub fn parse_conventional_commit(message: &'a str) -> Option<Self> {
        let commit = ConventionalCommitParser::parse(Rule::conventional_commit, message)
            .ok()?
            .next()
            .unwrap();

        let mut result = Self {
            type_: ChangeType::Fix,
            breaking: BreakingInfo::NotBreaking,
            scope: None,
            description: "",
            body: None,
        };

        for commit_part in commit.into_inner() {
            match commit_part.as_rule() {
                Rule::feat => result.type_ = ChangeType::Feature,
                Rule::fix => result.type_ = ChangeType::Fix,
                Rule::custom_type => result.type_ = ChangeType::Custom(commit_part.as_str()),
                Rule::breaking_flag => result.breaking = BreakingInfo::Breaking,
                Rule::scope => result.scope = Some(commit_part.as_str()),
                Rule::description => result.description = commit_part.as_str(),
                Rule::body => result.body = Some(commit_part.as_str()),
                Rule::footer => {
                    let mut is_breaking = false;
                    let mut footer_content: &str = "";
                    for footer_part in commit_part.into_inner() {
                        match footer_part.as_rule() {
                            Rule::breaking_change_token => is_breaking = true,
                            Rule::footer_value => footer_content = footer_part.as_str(),
                            _ => (),
                        }
                    }
                    if is_breaking {
                        result.breaking = BreakingInfo::BreakingWithDescription(footer_content);
                    }
                }
                _ => (),
            }
        }

        Some(result)
    }
}
