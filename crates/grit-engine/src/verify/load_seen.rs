// load_seen.rs — read keys of objects proven by earlier verify runs.
// Why: missing or damaged cache just means those objects get reread.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::store::paths::grit_dir;
use std::collections::BTreeSet;
use std::path::Path;

pub(super) fn load_seen(root: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(grit_dir(root).join("verified"))
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()
}
