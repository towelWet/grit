// target_entries.rs — the entries a restore should produce for one saved state, with the object ids that hold their bytes.
// Why: the plan, the apply step, and the final verify compare against the same map, and apply must read the bytes that really exist.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::store::read::{commit_entries, CommitEntries};
use crate::Result;
use std::path::Path;

pub fn target_entries(root: &Path, hash: &Hash) -> Result<CommitEntries> {
    commit_entries(root, hash) // legacy ids are comparable; their bytes stay under the stored object ids
}
