// current.rs — scan the working tree for restore planning and verification, using the same config and cache as snapshot.
// Why: restore must see exactly the paths a snapshot would see, or it would mis-plan removals.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::config::GritConfig;
use crate::index::Index;
use crate::workspace::{scan as scan_workspace, WorkspaceEntry};
use crate::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub fn scan(root: &Path, paths: Option<&[String]>) -> Result<BTreeMap<String, WorkspaceEntry>> {
    let config = GritConfig::load(root)?;
    let ignore = crate::ignore::IgnoreRules::new(&config.save.exclude); // same excludes as snapshot
    let index = Index::load(root); // stat cache: files untouched since the last snapshot are not reread
    scan_workspace(root, &ignore, paths, &index)
}
