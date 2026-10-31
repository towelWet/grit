// mod.rs — the stat cache: per-path content id plus the stat that proves the id is still valid.
// Why: unchanged files can be recognized from size, mtime, and inode alone, so neither snapshot nor status must read them.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod file_stat;
mod load;
mod lookup;
mod index_path;
mod now_ns;
mod save;
mod updated;

pub use file_stat::FileStat;
pub use now_ns::now_ns;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: String,
    pub hash: Hash,
    pub mode: u32,
    #[serde(default)] // absent in indexes written before symlink caching
    pub symlink_target: Option<String>,
    #[serde(default)] // stat fields are zero in entries that never cached a file
    pub size: u64,
    #[serde(default)]
    pub mtime_ns: u64, // modification time in nanoseconds since the Unix epoch
    #[serde(default)]
    pub inode: u64, // identity check: a replaced file gets a new inode even with equal size and mtime
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    pub entries: HashMap<String, IndexEntry>,
    #[serde(default)] // zero means "no racy-window guard" and such an index is never trusted
    pub scanned_ns: u64, // when the scan that wrote these entries began
}
