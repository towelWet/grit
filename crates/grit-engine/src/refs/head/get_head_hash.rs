// get_head_hash.rs — one item split out of head.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn get_head_hash(root: &Path) -> Result<Hash> {
    let head = get_head(root)?;
    if let Some(branch) = head.strip_prefix("ref: refs/heads/") {
        super::super::get_branch_ref(root, branch)
    } else {
        Hash::from_hex(&head)
    }
}
