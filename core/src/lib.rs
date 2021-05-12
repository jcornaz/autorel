#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChangeType {
    Fix,
    Feature,
    Breaking,
}

pub fn parse_commit_message(message: &str) -> Option<ChangeType> {
    if message.contains("BREAKING CHANGE:") {
        return Some(ChangeType::Breaking);
    }

    match message.splitn(2, ':').next() {
        Some(type_and_scope) => parse_type_and_scope(type_and_scope),
        None => None,
    }
}

fn parse_type_and_scope(type_and_scope: &str) -> Option<ChangeType> {
    if type_and_scope.ends_with('!') {
        return Some(ChangeType::Breaking);
    }

    let commit_type = type_and_scope
        .splitn(2, '(')
        .next()
        .unwrap_or(type_and_scope);

    match commit_type {
        "feat" => Some(ChangeType::Feature),
        "fix" => Some(ChangeType::Fix),
        _ => None,
    }
}
