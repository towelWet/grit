// is_ignored.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl IgnoreRules {
    pub fn is_ignored(&self, path: &Path) -> bool {
        if generated::generated(path) {
            return true; // rebuildable output, at any depth, in every repo
        }
        if self.inner.is_match(path) {
            return true;
        }
        let value = path.to_string_lossy();
        self.prefixes
            .iter()
            .any(|prefix| value == *prefix || value.starts_with(&format!("{prefix}/")))
    }
}
