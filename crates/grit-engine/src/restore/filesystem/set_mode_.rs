// set_mode_.rs — one item split out of filesystem.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(not(unix))]
pub fn set_mode(_: &Path, _: u32) -> Result<()> {
    Ok(())
}
