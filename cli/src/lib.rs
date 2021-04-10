#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![warn(clippy::pedantic)]

#[macro_use]
extern crate clap;

use clap::AppSettings;
pub use clap::Clap;

#[derive(Clap)]
#[clap(name = "autorel", version = crate_version ! (), author = crate_authors ! (), about = "Software release automation")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Command;
