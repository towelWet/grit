// impl_default.rs — one item split out of defaults.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

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
