#![allow(clippy::upper_case_acronyms)]
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Change {
    pub type_: ChangeType,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeType {
    Fix,
    Feature,
    Breaking,
}

impl Change {
    pub fn parse_commit_message(message: &str) -> Option<Self> {
        let commit = ConventionalCommitParser::parse(Rule::conventional_commit, message)
            .ok()?
            .next()
            .unwrap();

        let mut type_: Option<ChangeType> = None;

        for commit_part in commit.into_inner() {
            match commit_part.as_rule() {
                Rule::feat => type_ = Some(ChangeType::Feature),
                Rule::fix => type_ = Some(ChangeType::Fix),
                Rule::breaking_flag => {
                    return Some(Change {
                        type_: ChangeType::Breaking,
                    })
                }
                Rule::footer => {
                    if let Some(Rule::breaking_change_token) =
                        commit_part.into_inner().next().map(|it| it.as_rule())
                    {
                        return Some(Change {
                            type_: ChangeType::Breaking,
                        });
                    }
                }
                _ => (),
            }
        }

        type_.map(|type_| Self { type_ })
    }
}

#[derive(Parser)]
#[grammar = "conventional_commit.pest"]
struct ConventionalCommitParser;
