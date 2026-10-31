// types.rs — the inputs and outputs of one snapshot.
// Why: callers (save, restore's safety save, tests) share one shape, and `commit_hash` is optional because a clean tree records nothing.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::ignore::IgnoreRules;
use crate::Hash;
use std::collections::BTreeMap;
use std::path::Path;

pub struct SnapshotOptions<'a> {
    pub root: &'a Path,
    pub author: &'a str,
    pub message: &'a str,
    pub paths: Option<&'a [String]>, // None = whole repository; Some = only these paths
    pub ignore: &'a IgnoreRules,
    pub metadata: &'a BTreeMap<String, String>,
}

pub struct SnapshotResult {
    pub commit_hash: Option<Hash>, // None when nothing changed: no commit is created
    pub tree_hash: Option<Hash>,   // None for the same reason
    pub object_count: u64,
    pub bytes_added: u64,
    pub bytes_reused: u64,
    pub changed_files: Vec<String>,
    pub modified: Vec<String>,
    pub added: Vec<String>,
    pub deleted: Vec<String>,
    pub renamed: Vec<String>,
}

#[derive(Default)]
pub(crate) struct ObjectStats {
    pub count: u64,  // objects newly written
    pub added: u64,  // bytes of those objects (raw size)
    pub reused: u64, // bytes of objects that already existed
}
