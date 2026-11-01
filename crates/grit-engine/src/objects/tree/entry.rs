// entry.rs — one item split out of tree.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Tree {
    pub fn entry(&self, name: &str) -> Option<&TreeEntry> {
        self.entries.get(name)
    }
}
