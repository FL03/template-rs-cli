/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::args::*;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Cmd>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn command(&self) -> Option<&Cmd> {
        self.command.as_ref()
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

impl core::fmt::Display for Cli {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
