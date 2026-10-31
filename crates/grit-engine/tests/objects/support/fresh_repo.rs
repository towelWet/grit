// fresh_repo.rs — a new, empty repository in a temporary directory.
// Why: every test starts from known state and the directory disappears when the test ends.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
pub fn fresh_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap(); // removed when the test ends
    grit_engine::Repository::init(dir.path()).unwrap();
    dir
}
