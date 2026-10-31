use super::{collect, GcResult};
use crate::store::paths::objects_dir;
use crate::{GritError, Hash, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub(super) fn unreachable(
    root: &Path,
    reachable: &HashSet<Hash>,
    recovery: &Path,
) -> Result<GcResult> {
    let objects = objects_dir(root);
    let mut result = GcResult::default();
    if !objects.exists() {
        return Ok(result);
    }
    for entry in walkdir::WalkDir::new(&objects) {
        let entry = entry?;
        if !entry.file_type().is_file() || crate::store::is_appledouble(entry.path()) {
            continue; // dirs + macOS "._" sidecars on exFAT are not objects
        }
        let hash = collect::hash_from_path(&objects, entry.path())?;
        if reachable.contains(&hash) {
            result.kept += 1;
        } else {
            move_to_recovery(&objects, entry.path(), recovery, &mut result)?;
        }
    }
    Ok(result)
}

fn move_to_recovery(
    objects: &Path,
    path: &Path,
    recovery: &Path,
    result: &mut GcResult,
) -> Result<()> {
    let relative = path.strip_prefix(objects).map_err(|_| bad_path())?;
    let destination = recovery.join(relative);
    fs::create_dir_all(destination.parent().unwrap())?;
    result.bytes_freed += path.metadata()?.len();
    fs::rename(path, destination)?;
    result.removed += 1;
    Ok(())
}

fn bad_path() -> GritError {
    GritError::Internal("object path".into())
}
