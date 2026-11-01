// undone.rs — one item split out of safety_assertions.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn undone(root: &Path) {
    assert_eq!(fs::read(root.join("exact.bin")).unwrap(), [9, 8, 7]);
    assert!(!root.join("deleted.txt").exists());
    assert!(root.join("new-name.txt").is_file());
    assert!(!root.join("empty").exists());
    assert!(!root.join("exact-link").exists());
    assert_eq!(mode(&root.join("bin/run.sh")), 0o644);
}
