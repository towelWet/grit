// stall_ignore.rs — ignored and generated folders stay out of snapshots.
// Why: grit once hashed whole media libraries and nested build output, stalling every session.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run_ok, write};

#[test]
fn gritignore_and_generated_folders_stay_out() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, ".gritignore", "# runtime media\ndata/\n/cache\n*.log\n");
    for hidden in ["data/media/video.mp4", "cache/x.bin", "sub/run.log", "app/.build/out.o",
                   ".build-tmp/f/1.png", "tool/.venv/bin/py", "lib/__pycache__/a.pyc"] {
        write(root, hidden, "huge");
    }
    write(root, "src/main.rs", "fn main() {}");
    let saved = run_ok(root, &["snapshot", "-m", "first", "--format", "toml"]);
    assert!(saved.contains("src/main.rs"), "{saved}");
    for name in ["video.mp4", "x.bin", "run.log", "out.o", "1.png", "bin/py", "a.pyc"] {
        assert!(!saved.contains(name), "{name} leaked: {saved}"); // never hashed or stored
    }
}
