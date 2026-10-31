// updated.rs — fold a finished scan into the stat cache, replacing only the scanned scope.
// Why: a path-scoped snapshot must not forget cache entries outside its scope.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::{Index, IndexEntry};
use crate::workspace::{selected, WorkspaceEntry};
use std::collections::BTreeMap;

impl Index {
    pub fn updated(
        mut previous: Index,
        scanned: &BTreeMap<String, WorkspaceEntry>,
        selections: Option<&[String]>,
        scanned_ns: u64,
    ) -> Self {
        previous.entries.retain(|path, _| !selected(path, selections)); // in-scope entries are rebuilt below
        for (path, item) in scanned {
            if let Some(stat) = item.stat {
                // Only regular files are cached; directories and symlinks are cheap to describe each time.
                previous.entries.insert(
                    path.clone(),
                    IndexEntry {
                        path: path.clone(),
                        hash: item.hash,
                        mode: item.mode,
                        symlink_target: None,
                        size: stat.size,
                        mtime_ns: stat.mtime_ns,
                        inode: stat.inode,
                    },
                );
            }
        }
        previous.scanned_ns = scanned_ns; // the racy guard for the next lookup uses this scan's start time
        previous
    }
}
