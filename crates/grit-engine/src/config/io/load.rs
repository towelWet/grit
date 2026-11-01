// load.rs — one item split out of io.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl GritConfig {
    pub fn load(root: &Path) -> Result<Self> {
        let path = config_path(root);
        let mut config: Self = if path.exists() {
            toml::from_str(&fs::read_to_string(path)?)?
        } else {
            Self::default()
        };
        config.save.exclude.extend(super::super::gritignore::gritignore(root)); // memory only: init alone calls save
        Ok(config)
    }
}
