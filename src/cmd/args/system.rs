/*
    Appellation: system <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
pub struct SystemCommand {
    #[clap(subcommand)]
    pub args: Option<SystemOpts>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    Subcommand,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SystemOpts {
    #[default]
    Config {
        #[clap(long, short, default_value_t = String::from("Proton.toml"))]
        path: String,
    },
}

impl SystemOpts {
    pub fn config(path: String) -> Self {
        Self::Config { path }
    }
}
