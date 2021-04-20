use serde_derive::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Default)]
pub struct Hooks {
    verify: Option<String>,
    prepare: Option<String>,
    publish: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_can_be_empty() {
        assert_eq!(toml::from_str::<Config>(""), Ok(Config::default()));
    }

    #[test]
    fn parse_verify_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            verify = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.verify, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_prepare_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            prepare = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.prepare, Some(String::from("myscript.sh")))
    }

    #[test]
    fn parse_publish_hook() {
        let config: Config = toml::from_str(
            r#"
            [hooks]
            publish = "myscript.sh"
        "#,
        )
        .expect("Failed to parse config");

        assert_eq!(config.hooks.publish, Some(String::from("myscript.sh")))
    }
}
