// stall_edits.rs — snapshots succeed while another process edits and deletes files.
// Why: "changed during snapshot" errors made grit unusable alongside agents and live logs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run_ok, write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn edits_and_deletes_during_snapshots_do_not_fail_them() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().to_path_buf();
    run_ok(&root, &["init"]);
    for i in 0..100 {
        write(&root, &format!("f/{i}.txt"), "start");
    }
    let stop = Arc::new(AtomicBool::new(false));
    let (target, flag) = (root.clone(), stop.clone());
    let writer = std::thread::spawn(move || {
        let mut round = 0;
        while !flag.load(Ordering::Relaxed) {
            round += 1;
            write(&target, &format!("f/{}.txt", round % 100), &format!("edit {round}"));
            let _ = std::fs::remove_file(target.join(format!("f/{}.txt", (round + 50) % 100)));
        }
    });
    for i in 0..3 {
        run_ok(&root, &["snapshot", "-m", &format!("s{i}")]); // run_ok asserts success
    }
    stop.store(true, Ordering::Relaxed);
    writer.join().unwrap();
    assert!(run_ok(&root, &["verify", "--full"]).contains(" 0 corrupt")); // stored history stays sound
}
