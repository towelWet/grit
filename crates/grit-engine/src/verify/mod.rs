mod check_entry;
mod load_seen;
mod model;
mod object;
mod save_seen;
mod seen_key;

pub use model::VerifyResult;

use crate::gc::collect_reachable;
use crate::store::paths::objects_dir;
use crate::Result;
use std::path::Path;

// full = reread every object. Otherwise objects already proven and untouched on disk are
// trusted: they are immutable, and rehashing a media history took 19+ minutes per run.
pub fn verify(root: &Path, full: bool) -> Result<VerifyResult> {
    let objects = objects_dir(root);
    let seen = if full { Default::default() } else { load_seen::load_seen(root) };
    let mut proven = std::collections::BTreeSet::new();
    let mut result = VerifyResult::default();
    if !objects.exists() {
        return Ok(result);
    }
    for entry in walkdir::WalkDir::new(&objects) {
        let entry = entry?;
        if !entry.file_type().is_file() || crate::store::is_appledouble(entry.path()) {
            continue; // dirs + macOS "._" sidecars on exFAT are not objects
        }
        check_entry::check_entry(&objects, &entry, &seen, &mut proven, &mut result)?;
    }
    let _ = save_seen::save_seen(root, &proven); // lost cache only means a slower next run, never wrong result
    result.reachable = collect_reachable(root)?.len() as u64;
    result.unreachable = result.total_objects.saturating_sub(result.reachable);
    Ok(result)
}
