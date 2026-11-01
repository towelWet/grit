// symlink.rs — one item split out of support.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(unix)]
pub fn symlink(target: &str, path: &Path) {
    std::os::unix::fs::symlink(target, path).unwrap();
}
