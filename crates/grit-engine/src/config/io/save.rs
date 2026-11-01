// save.rs — one item split out of io.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl GritConfig {

    pub fn save(&self, root: &Path) -> Result<()> {
        fs::write(config_path(root), toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
