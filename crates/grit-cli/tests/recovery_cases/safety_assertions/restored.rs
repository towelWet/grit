// restored.rs — one item split out of safety_assertions.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn restored(root: &Path, original: [u8; 5]) {
    assert_eq!(fs::read(root.join("exact.bin")).unwrap(), original);
    assert_eq!(read(root, "deleted.txt"), "restore me\n");
    assert!(root.join("old-name.txt").is_file());
    assert!(!root.join("new-name.txt").exists());
    assert!(root.join("empty/directory").is_dir());
    assert_eq!(mode(&root.join("bin/run.sh")), 0o755);
    assert_eq!(
        fs::read_link(root.join("exact-link")).unwrap(),
        Path::new("exact.bin")
    );
}
