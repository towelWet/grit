// same_content.rs — one item split out of classify.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn same_content(left: &TreeEntry, right: &TreeEntry) -> bool {
    left.kind == right.kind
        && left.hash == right.hash
        && left.mode == right.mode
        && left.target == right.target
}
