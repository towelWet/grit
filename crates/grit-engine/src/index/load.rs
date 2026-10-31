// load.rs — read the stat cache, treating a missing or corrupt file as an empty cache.
// Why: the index is only an accelerator; losing it costs one full rehash, never correctness.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::index_path::index_path;
use super::Index;
use std::fs;
use std::path::Path;

impl Index {
    pub fn load(root: &Path) -> Self {
        let Ok(text) = fs::read_to_string(index_path(root)) else {
            return Self::default(); // no cache yet: every file is hashed once
        };
        toml::from_str(&text).unwrap_or_default() // corrupt cache: discard it rather than trust it
    }
}
