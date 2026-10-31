use super::{CoreConfig, GritConfig, RestoreConfig, SaveConfig};

pub fn hash() -> String {
    "blake3".into()
}
pub fn compression() -> String {
    "zstd".into()
}
pub fn compression_level() -> u32 {
    3
}
pub fn enabled() -> bool {
    true
}

impl Default for GritConfig {
    fn default() -> Self {
        Self {
            core: CoreConfig {
                hash: hash(),
                compression: compression(),
                compression_level: compression_level(),
            },
            save: SaveConfig {
                exclude: [".grit/", ".git/", "target/", "node_modules/", ".DS_Store"]
                    .map(String::from)
                    .to_vec(),
            },
            restore: RestoreConfig {
                safety_save: true,
                confirm_overwrite: true,
            },
        }
    }
}
