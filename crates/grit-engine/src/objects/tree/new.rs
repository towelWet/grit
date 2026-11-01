// new.rs — one item split out of tree.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Tree {
    pub fn new(e: BTreeMap<String, TreeEntry>) -> Self {
        Self { entries: e }
    }
}
