#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![warn(clippy::pedantic)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

mod cli;

fn main() {
    cli::parse();

    run_script_if_exists(".release/verify.sh".into());
    run_script_if_exists(".release/prepare.sh".into());
    run_script_if_exists(".release/publish.sh".into());
}

fn run_script_if_exists(script: PathBuf) -> bool {
    if script.exists() {
        run(script)
    } else {
        true
    }
}

fn run(script: impl AsRef<OsStr>) -> bool {
    match Command::new(script).status() {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}
