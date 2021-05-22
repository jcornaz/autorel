use pest::Parser;

use super::*;

#[derive(Parser)]
#[grammar = "conventional_commit.pest"]
struct ConventionalCommitParser;

pub(crate) fn parse(commit_msg: &str) -> Option<Change> {
    let commit = ConventionalCommitParser::parse(Rule::conventional_commit, commit_msg)
        .ok()?
        .next()?;

    let mut result = Change::new(ChangeType::Fix, "");

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
                    match &mut result.breaking {
                        BreakingInfo::NotBreaking | BreakingInfo::Breaking => {
                            result.breaking =
                                BreakingInfo::BreakingWithDescriptions(vec![footer_content]);
                        }
                        BreakingInfo::BreakingWithDescriptions(infos) => {
                            infos.push(footer_content);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    Some(result)
}
