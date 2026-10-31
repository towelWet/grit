use super::support::{mode, set_mode, symlink, write};
use std::fs;
use std::path::Path;

pub fn populate(root: &Path) {
    for index in 0..500 {
        write(
            root,
            &format!("src/file-{index:03}.txt"),
            &format!("A-{index}\n"),
        );
    }
    fs::create_dir_all(root.join("empty/directory")).unwrap();
    write(root, "bin/run.sh", "#!/bin/sh\necho grit\n");
    set_mode(&root.join("bin/run.sh"), 0o755);
    symlink("../src/file-000.txt", &root.join("bin/link"));
}

pub fn assert_restored(root: &Path) {
    assert_eq!(
        fs::read_to_string(root.join("src/file-000.txt")).unwrap(),
        "A-0\n"
    );
    assert!(root.join("src/file-200.txt").exists());
    assert!(!root.join("src/renamed-200.txt").exists());
    assert!(!root.join("new/file-000.txt").exists());
    assert!(root.join("empty/directory").is_dir());
    assert_eq!(mode(&root.join("bin/run.sh")), 0o755);
    assert_eq!(
        fs::read_link(root.join("bin/link")).unwrap(),
        Path::new("../src/file-000.txt")
    );
}
