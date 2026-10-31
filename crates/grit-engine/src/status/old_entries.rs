// old_entries.rs — HEAD's normalized entries, or an empty map when the repository has no commit yet.
// Why: a fresh repository must show every file as added rather than failing on a missing HEAD.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::TreeEntry;
use crate::store::read::commit_entries;
use crate::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub fn old_entries(root: &Path) -> Result<BTreeMap<String, TreeEntry>> {
    let Ok(hash) = crate::refs::get_head_hash(root) else {
        return Ok(BTreeMap::new()); // no HEAD: nothing is recorded yet
    };
    Ok(commit_entries(root, &hash)?.entries) // bytes are not needed for status, only ids
}
