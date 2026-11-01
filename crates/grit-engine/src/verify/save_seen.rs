// save_seen.rs — record objects proven this run for next verify.
// Why: only keys seen this run are kept, so gc'd objects drop out of cache.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::store::paths::grit_dir;
use crate::store::write::atomic_write;
use crate::Result;
use std::collections::BTreeSet;
use std::path::Path;

pub(super) fn save_seen(root: &Path, proven: &BTreeSet<String>) -> Result<()> {
    let lines: Vec<&str> = proven.iter().map(String::as_str).collect();
    atomic_write(&grit_dir(root).join("verified"), lines.join("\n").as_bytes())
}
