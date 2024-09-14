/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Settings {
    pub mode: Mode,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumString,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[serde(alias = "dev")]
    Development,
    #[serde(alias = "stag")]
    Staging,
    #[serde(alias = "prod")]
    Production,
}
