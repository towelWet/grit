// lookup.rs — the cached content id for a path, but only when the cache can be trusted.
// Why: a stat match is proof of unchanged content only if the cached entry was recorded before a same-tick write could occur.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::{FileStat, Index};
use crate::hash::Hash;

impl Index {
    pub fn lookup(&self, path: &str, stat: &FileStat) -> Option<Hash> {
        let entry = self.entries.get(path)?;
        let unchanged = entry.size == stat.size
            && entry.mtime_ns == stat.mtime_ns
            && entry.inode == stat.inode;
        // Racy guard (as in git): an mtime at or after the scan start could be a write that landed in
        // the same timestamp tick as our read, so such an entry is never trusted; it is simply rehashed.
        let settled = entry.mtime_ns < self.scanned_ns;
        (unchanged && settled).then_some(entry.hash)
    }
}
