// tree_entry.rs — the tree-entry view of a scanned path, named by its final path component.
// Why: stored trees key entries by full path, and the name field is the human-readable basename.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::WorkspaceEntry;
use crate::objects::TreeEntry;
use crate::Result;

impl WorkspaceEntry {
    pub fn tree_entry(&self, path: &str) -> Result<TreeEntry> {
        Ok(TreeEntry {
            name: path.rsplit('/').next().unwrap_or(path).to_string(), // basename of the path
            kind: self.kind.clone(),
            hash: self.hash, // already computed by the scan (or the stat cache)
            mode: self.mode,
            target: self.target.clone(),
        })
    }
}
