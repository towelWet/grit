// empty.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl IgnoreRules {
    pub fn empty() -> Self {
        Self {
            inner: GlobSet::empty(),
            prefixes: Vec::new(),
        }
    }
}
