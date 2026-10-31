use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GritConfig {
    #[serde(default)]
    pub core: CoreConfig,
    #[serde(default)]
    pub save: SaveConfig,
    #[serde(default)]
    pub restore: RestoreConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreConfig {
    #[serde(default = "super::defaults::hash")]
    pub hash: String,
    #[serde(default = "super::defaults::compression")]
    pub compression: String,
    #[serde(default = "super::defaults::compression_level")]
    pub compression_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveConfig {
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestoreConfig {
    #[serde(default = "super::defaults::enabled")]
    pub safety_save: bool,
    #[serde(default = "super::defaults::enabled")]
    pub confirm_overwrite: bool,
}
