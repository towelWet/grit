// parent_root.rs — the parent's nested root tree, when the parent uses the nested format.
// Why: only a nested parent has reusable per-directory trees; anything older is rebuilt from scratch.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::commit::FORMAT_NESTED;
use crate::store::read::read_commit;
use crate::Result;
use std::path::Path;

pub fn parent_root(root: &Path, parent: Option<&Hash>) -> Result<Option<Hash>> {
    let Some(hash) = parent else {
        return Ok(None); // first snapshot: nothing to reuse
    };
    let commit = read_commit(root, hash)?;
    Ok((commit.format == FORMAT_NESTED).then_some(commit.root_tree)) // flat parents are not reusable
}
