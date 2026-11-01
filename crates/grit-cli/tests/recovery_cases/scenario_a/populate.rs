// populate.rs — one item split out of scenario_a.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

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
