// move_aside.rs — one item split out of backup.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn move_aside(root: &Path, relative: &str, backup: &Path) -> Result<()> {
    validate_relative(relative)?;
    let source = root.join(relative);
    if fs::symlink_metadata(&source).is_err() {
        return Ok(());
    }
    let destination = backup.join(relative);
    fs::create_dir_all(destination.parent().unwrap())?;
    if fs::symlink_metadata(&source)?.is_dir() && destination.exists() {
        fs::rename(source, destination.join(".grit-directory"))?;
    } else {
        fs::rename(source, destination)?;
    }
    Ok(())
}
