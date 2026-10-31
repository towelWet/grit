// selection.rs — integration tests for revision `~N` syntax and --path symlink guarding.
// Why: both bugs (2026-10-29) failed quietly or confusingly in daily use:
// `<save>~1` was "Save not found", and --path through a symlink printed "(clean)".
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
#[path = "selection/ancestor_rev.rs"]
mod ancestor_rev; // <save>~1 / ~ / ~N resolve via first parent; past root errors
#[path = "selection/symlink_path.rs"]
mod symlink_path; // --path crossing a symlink errors; selecting the link itself is fine
#[path = "objects/support/mod.rs"]
#[allow(dead_code, unused_imports)] // shared fixtures; this harness uses only fresh_repo + save_snapshot
mod support;
