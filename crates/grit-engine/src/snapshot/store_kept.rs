// store_kept.rs — make every kept path's object exist, in parallel, once a change is known to exist.
// Why: only a snapshot that will create a commit may write objects, so a clean tree leaves no orphans behind.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::store_entry::store_entry;
use super::types::ObjectStats;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;

pub fn store_kept(root: &Path, kept: &BTreeMap<String, WorkspaceEntry>) -> Result<ObjectStats> {
    kept.par_iter() // misses are re-read and written; hits cost one stat each
        .map(|(path, item)| store_entry(root, path, item))
        .try_reduce(ObjectStats::default, |left, right| Ok(left + right)) // sums per-worker counts
}
