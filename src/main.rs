#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![warn(clippy::pedantic)]

use autorel_cli::{Clap, Command};

fn main() {
    let _: Command = Command::parse();
}
