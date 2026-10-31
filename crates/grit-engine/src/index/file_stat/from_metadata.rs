// from_metadata.rs — read the stat fields from filesystem metadata.
// Why: the stat is captured before a file is read, so a later write always shows up as a changed stat.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::FileStat;
use std::fs::Metadata;
use std::time::UNIX_EPOCH;

impl FileStat {
    pub fn from_metadata(metadata: &Metadata) -> Self {
        let mtime_ns = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|since| since.as_nanos() as u64)
            .unwrap_or(u64::MAX); // unknown mtime must never match a cached entry, so use the maximum
        #[cfg(unix)]
        let inode = {
            use std::os::unix::fs::MetadataExt;
            metadata.ino() // identity of the file on this volume
        };
        #[cfg(not(unix))]
        let inode = 0u64; // no inode here: matching falls back to size and mtime only
        Self {
            size: metadata.len(),
            mtime_ns,
            inode,
        }
    }
}
