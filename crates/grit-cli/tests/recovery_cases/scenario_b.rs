use super::support::write;
use std::fs;
use std::path::Path;

pub fn mutate(root: &Path) {
    for index in 0..200 {
        write(
            root,
            &format!("src/file-{index:03}.txt"),
            &format!("B-{index}\n"),
        );
    }
    for index in 200..240 {
        fs::rename(
            root.join(format!("src/file-{index:03}.txt")),
            root.join(format!("src/renamed-{index:03}.txt")),
        )
        .unwrap();
    }
    for index in 240..260 {
        fs::rename(
            root.join(format!("src/file-{index:03}.txt")),
            root.join(format!("deleted-{index:03}.txt")),
        )
        .unwrap();
    }
    for index in 0..30 {
        write(
            root,
            &format!("new/file-{index:03}.txt"),
            &format!("new-{index}\n"),
        );
    }
}

pub fn assert_restored(root: &Path) {
    assert_eq!(
        fs::read_to_string(root.join("src/file-000.txt")).unwrap(),
        "B-0\n"
    );
    assert!(root.join("src/renamed-200.txt").exists());
    assert!(root.join("new/file-029.txt").exists());
}
