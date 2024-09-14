/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::opts::Options;

pub(crate) mod opts;

pub mod args;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub fn new() -> Cli {
    Cli::parse()
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Options>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}

impl core::fmt::Display for Cli {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            command: None,
            verbose: false,
        }
    }
}
