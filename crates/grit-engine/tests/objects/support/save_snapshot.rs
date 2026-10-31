// save_snapshot.rs — run one whole-repository snapshot with default options and return its commit id.
// Why: nested-tree tests need real snapshots, and the same options must be used every time.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use grit_engine::config::GritConfig;
use grit_engine::ignore::IgnoreRules;
use grit_engine::snapshot::{create_snapshot, SnapshotOptions};
use grit_engine::{Hash, Result};
use std::collections::BTreeMap;
use std::path::Path;

pub fn save_snapshot(root: &Path, message: &str) -> Result<Hash> {
    let config = GritConfig::default(); // same defaults the CLI uses, so .grit/ itself is excluded
    let ignore = IgnoreRules::new(&config.save.exclude);
    let metadata = BTreeMap::new();
    let result = create_snapshot(&SnapshotOptions {
        root,
        author: "test",
        message,
        paths: None,
        ignore: &ignore,
        metadata: &metadata,
    })?;
    Ok(result.commit_hash.expect("a changed tree must produce a commit"))
}
