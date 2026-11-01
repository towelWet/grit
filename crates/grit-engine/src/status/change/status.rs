// status.rs — one item split out of change.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn status(old: &TreeEntry, new: &TreeEntry) -> Option<&'static str> {
    if old.kind != new.kind {
        Some("type-changed")
    } else if old.target != new.target {
        Some("symlink-changed")
    } else if old.hash != new.hash {
        Some("modified")
    } else if old.mode != new.mode {
        Some("permission-changed")
    } else {
        None
    }
}
