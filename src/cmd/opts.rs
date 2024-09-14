/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::args::*;

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Options {
    Build,
    Platform(PlatformCommand),
    System(SystemCommand),
}

impl Options {
    pub fn from_platform(platform: PlatformCommand) -> Self {
        Self::Platform(platform)
    }

    pub fn from_system(system: SystemCommand) -> Self {
        Self::System(system)
    }
}
