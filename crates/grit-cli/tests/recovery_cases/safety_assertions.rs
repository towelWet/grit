use super::support::mode;
use std::fs;
use std::path::Path;

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

pub fn undone(root: &Path) {
    assert_eq!(fs::read(root.join("exact.bin")).unwrap(), [9, 8, 7]);
    assert!(!root.join("deleted.txt").exists());
    assert!(root.join("new-name.txt").is_file());
    assert!(!root.join("empty").exists());
    assert!(!root.join("exact-link").exists());
    assert_eq!(mode(&root.join("bin/run.sh")), 0o644);
}

fn read(root: &Path, path: &str) -> String {
    fs::read_to_string(root.join(path)).unwrap()
}
