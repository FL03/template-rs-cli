/*
    Appellation: args <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{platform::*, system::*};

pub(crate) mod platform;
pub(crate) mod system;

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
pub enum Cmd {
    Build,
    Platform(PlatformCmd),
    System(SystemCommand),
}

impl Cmd {
    pub fn from_platform(platform: PlatformCmd) -> Self {
        Self::Platform(platform)
    }

    pub fn from_system(system: SystemCommand) -> Self {
        Self::System(system)
    }
}
