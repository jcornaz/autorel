pub trait Action {
    type Error: std::error::Error;

    fn title(&self, version_str: &str) -> String;
    fn execute(&self, version_str: &str) -> Result<(), Self::Error>;
    fn dry_run(&self, version_str: &str) -> Vec<String>;
}
