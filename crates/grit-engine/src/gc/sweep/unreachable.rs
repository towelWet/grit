// unreachable.rs — one item split out of sweep.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(in super::super::super) fn unreachable(
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
