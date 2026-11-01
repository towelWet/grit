// store_changed.rs — write changed objects, then make tree map name what was really stored.
// Why: keeps create.rs a readable sequence; races are resolved here before trees are built.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::apply_raced::apply_raced;
use super::prepare::Prepared;
use super::store_kept::store_kept;
use super::types::ObjectStats;
use crate::Result;
use std::path::Path;
use std::sync::Mutex;

pub(super) fn store_changed(root: &Path, prepared: &mut Prepared) -> Result<ObjectStats> {
    let raced = Mutex::new(Vec::new()); // files that changed or vanished after the scan
    let stats = store_kept(root, &prepared.kept, &raced)?;
    apply_raced(&mut prepared.new, raced.into_inner().unwrap_or_default());
    Ok(stats)
}
