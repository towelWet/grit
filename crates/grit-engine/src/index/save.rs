// save.rs — write the stat cache atomically.
// Why: status may read the index while a snapshot writes it, so readers must never see a half-written file.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::index_path::index_path;
use super::Index;
use crate::store::write::atomic_write;
use crate::Result;
use std::path::Path;

impl Index {
    pub fn save(&self, root: &Path) -> Result<()> {
        let text = toml::to_string_pretty(self)?; // serialization errors surface as GritError::TomlSer
        atomic_write(&index_path(root), text.as_bytes()) // temp file + rename, so readers see old or new
    }
}
