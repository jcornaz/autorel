#![allow(clippy::upper_case_acronyms)]
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Change<'a> {
    pub type_: ChangeTypeWithDesc<'a>,
    pub scope: Option<&'a str>,
    pub description: &'a str,
    pub body: Option<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeTypeWithDesc<'a> {
    Fix,
    Feature,
    Breaking(Option<&'a str>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeType {
    Fix,
    Feature,
    Breaking,
}

impl From<ChangeTypeWithDesc<'_>> for ChangeType {
    fn from(type_: ChangeTypeWithDesc<'_>) -> Self {
        type_.without_description()
    }
}

impl ChangeTypeWithDesc<'_> {
    pub fn without_description(&self) -> ChangeType {
        match self {
            ChangeTypeWithDesc::Fix => ChangeType::Fix,
            ChangeTypeWithDesc::Feature => ChangeType::Feature,
            ChangeTypeWithDesc::Breaking(_) => ChangeType::Breaking,
        }
    }
}

impl<'a> Change<'a> {
    pub fn parse_conventional_commit(message: &'a str) -> Option<Self> {
        let commit = ConventionalCommitParser::parse(Rule::conventional_commit, message)
            .ok()?
            .next()
            .unwrap();

        let mut type_: Option<ChangeTypeWithDesc> = None;
        let mut scope: Option<&str> = None;
        let mut description: &str = "";
        let mut body: Option<&str> = None;

        for commit_part in commit.into_inner() {
            match commit_part.as_rule() {
                Rule::feat => type_ = Some(ChangeTypeWithDesc::Feature),
                Rule::fix => type_ = Some(ChangeTypeWithDesc::Fix),
                Rule::breaking_flag => type_ = Some(ChangeTypeWithDesc::Breaking(None)),
                Rule::scope => scope = Some(commit_part.as_str()),
                Rule::description => description = commit_part.as_str(),
                Rule::body => body = Some(commit_part.as_str()),
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
                        type_ = Some(ChangeTypeWithDesc::Breaking(Some(footer_content)));
                    }
                }
                _ => (),
            }
        }

        type_.map(|type_| Self {
            type_,
            scope,
            description,
            body,
        })
    }
}

#[derive(Parser)]
#[grammar = "conventional_commit.pest"]
struct ConventionalCommitParser;
