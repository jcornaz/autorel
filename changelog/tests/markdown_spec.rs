use autorel_chlg::{BreakingInfo, Change, ChangeLog, ChangeType};

#[test]
fn markdown_example() {
    let mut changelog = ChangeLog::default();

    changelog += Change {
        type_: ChangeType::Feature,
        scope: None,
        description: "Feature without scope",
        breaking: BreakingInfo::NotBreaking,
        body: None,
    };
    changelog += Change {
        type_: ChangeType::Feature,
        scope: Some("test-scope"),
        description: "Feature with scope",
        breaking: BreakingInfo::NotBreaking,
        body: None,
    };
    changelog += Change {
        type_: ChangeType::Fix,
        scope: Some("test-scope"),
        description: "Breaking fix",
        breaking: BreakingInfo::Breaking,
        body: None,
    };
    changelog += Change {
        type_: ChangeType::Feature,
        scope: None,
        description: "Breaking feature with more info",
        breaking: BreakingInfo::BreakingWithDescription("because!"),
        body: None,
    };

    let formated = format!("{}", changelog.markdown());

    assert_eq!(
        formated,
        r"### Breaking changes

* because!

#### test-scope

* Breaking fix


### Features

* Feature without scope
* Breaking feature with more info

#### test-scope

* Feature with scope


### Bug fixes

#### test-scope

* Breaking fix


"
    )
}
