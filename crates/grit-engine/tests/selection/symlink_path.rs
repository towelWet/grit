// symlink_path.rs — --path through a symlinked dir must error, not report "(clean)".
// Why: a skills dir -> an external volume was silently "clean" in home repo (2026-10-29).
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::fresh_repo;
use grit_engine::config::GritConfig;
use grit_engine::ignore::IgnoreRules;
use grit_engine::status::compute_status;
use grit_engine::GritError;
use std::fs;

#[test]
fn path_through_symlink_errors_but_link_itself_is_allowed() {
    let repo = fresh_repo();
    let root = repo.path();
    let outside = tempfile::tempdir().unwrap();               // real data lives elsewhere
    fs::write(outside.path().join("x.txt"), "data").unwrap();
    std::os::unix::fs::symlink(outside.path(), root.join("link")).unwrap();
    let ignore = IgnoreRules::new(&GritConfig::default().save.exclude);
    let through = vec!["link/x.txt".to_string()];
    let result = compute_status(root, &ignore, Some(&through));
    assert!(matches!(result, Err(GritError::SymlinkSelection(_)))); // loud, not "(clean)"
    let itself = vec!["link".to_string()];
    assert!(compute_status(root, &ignore, Some(&itself)).is_ok()); // link tracked as a link
}
