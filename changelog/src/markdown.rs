use std::fmt;
use std::fmt::{Display, Formatter};

use crate::changelog::{Scope, Section};
use crate::ChangeLog;

impl ChangeLog {
    pub fn markdown(&self) -> MarkdownChangelog {
        let mut scopes: Vec<&Option<Scope>> = self.scopes().collect();
        scopes.sort();

        MarkdownChangelog {
            breaking_changes: MarkdownChangelogSection {
                title: "Breaking changes",
                scopes: scopes.clone(),
                section: self.breaking_changes(),
            },
            features: MarkdownChangelogSection {
                title: "Features",
                scopes: scopes.clone(),
                section: self.features(),
            },
            fixes: MarkdownChangelogSection {
                title: "Bug fixes",
                scopes,
                section: self.fixes(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownChangelog<'a> {
    breaking_changes: MarkdownChangelogSection<'a>,
    features: MarkdownChangelogSection<'a>,
    fixes: MarkdownChangelogSection<'a>,
}

#[derive(Debug, Clone)]
struct MarkdownChangelogSection<'a> {
    title: &'a str,
    scopes: Vec<&'a Option<Scope>>,
    section: &'a Section,
}

impl Display for MarkdownChangelog<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.breaking_changes, self.features, self.fixes
        )
    }
}

impl Display for MarkdownChangelogSection<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.section.is_empty() {
            return Ok(());
        }
        write!(f, "### {}\n\n", self.title)?;

        let iter = self
            .scopes
            .iter()
            .flat_map(|scope| self.section.get(scope).map(|changes| (scope, changes)));

        for (scope, changes) in iter {
            if let Some(title) = scope {
                write!(f, "#### {}\n\n", title)?;
            }

            for change in changes {
                writeln!(f, "* {}", change)?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}
