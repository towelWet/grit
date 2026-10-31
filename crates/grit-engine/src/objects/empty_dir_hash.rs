// empty_dir_hash.rs — the id every directory entry carries in the flat (path-keyed) view.
// Why: scans describe a directory by one constant id, so flat maps compare equal across formats.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Tree;
use crate::hash::Hash;
use crate::Result;

pub fn empty_dir_hash() -> Result<Hash> {
    Ok(Hash::of(&serde_json::to_vec(&Tree::empty())?)) // same bytes the directory object is stored with
}
