/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, clap::Parser,
)]
pub struct PlatformCmd {
    #[clap(subcommand)]
    pub args: Option<PlatformOpts>,
}

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
    strum::EnumCount,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase", untagged)]
#[strum(serialize_all = "lowercase")]
pub enum PlatformOpts {
    Connect {
        #[clap(long, short)]
        target: Option<String>,
    },
}

/*
 ************* Implementations *************
*/

impl PlatformOpts {
    pub fn connect(target: Option<String>) -> Self {
        Self::Connect { target }
    }
}

impl core::fmt::Display for PlatformOpts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_command() {
        let args = PlatformOpts::Connect {
            target: Some("10".to_string()),
        };
        assert_eq!(args.to_string(), "connect");
        println!("{}", args);
    }
}
