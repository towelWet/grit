// no_change_result.rs — the result of a snapshot that found nothing to record.
// Why: a clean tree must not add a commit, so the result carries counts but no commit or tree id.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::types::{ObjectStats, SnapshotResult};

pub(super) fn no_change_result(stats: ObjectStats) -> SnapshotResult {
    SnapshotResult {
        commit_hash: None, // no commit was written
        tree_hash: None,   // no tree was written either
        object_count: stats.count,
        bytes_added: stats.added,
        bytes_reused: stats.reused,
        changed_files: Vec::new(),
        modified: Vec::new(),
        added: Vec::new(),
        deleted: Vec::new(),
        renamed: Vec::new(),
    }
}
