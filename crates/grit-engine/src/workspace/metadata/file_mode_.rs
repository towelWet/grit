// file_mode_.rs — one item split out of metadata.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(not(unix))]
pub fn file_mode(metadata: &fs::Metadata) -> u32 {
    if metadata.is_dir() {
        0o040755
    } else {
        0o100644
    }
}
