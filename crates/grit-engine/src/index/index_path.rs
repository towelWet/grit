// index_path.rs — where the stat cache lives inside .grit.
// Why: load and save must agree on one path, so it is computed in exactly one place.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::store::paths::grit_dir;
use std::path::{Path, PathBuf};

pub fn index_path(root: &Path) -> PathBuf {
    grit_dir(root).join("index").join("current.toml") // TOML, as older builds wrote it
}
