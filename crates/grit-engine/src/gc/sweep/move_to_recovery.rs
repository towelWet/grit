// move_to_recovery.rs — one item split out of sweep.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn move_to_recovery(
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
