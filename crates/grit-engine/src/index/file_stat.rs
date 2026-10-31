// file_stat.rs — the three stat fields that identify a file's content without reading it.
// Why: size, mtime, and inode change whenever content is rewritten, so equal stats mean equal bytes (modulo racy timestamps).
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod from_metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStat {
    pub size: u64,      // byte length at scan time
    pub mtime_ns: u64,  // last content change, nanoseconds since the Unix epoch
    pub inode: u64,     // filesystem identity; 0 where the platform has no inode
}
