use super::GritConfig;
use crate::store::paths::config_path;
use crate::Result;
use std::fs;
use std::path::Path;

impl GritConfig {
    pub fn load(root: &Path) -> Result<Self> {
        let path = config_path(root);
        let mut config: Self = if path.exists() {
            toml::from_str(&fs::read_to_string(path)?)?
        } else {
            Self::default()
        };
        config.save.exclude.extend(super::gritignore::gritignore(root)); // memory only: init alone calls save
        Ok(config)
    }

    pub fn save(&self, root: &Path) -> Result<()> {
        fs::write(config_path(root), toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
