// mutate.rs — one item split out of scenario_b.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

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
