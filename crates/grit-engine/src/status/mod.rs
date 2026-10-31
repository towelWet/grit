// mod.rs — status module: the public compute entry point and the model it returns.
// Why: callers see one stable surface while change detection and renames stay internal.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod change;
mod compute;
mod model;
mod old_entries;
mod rename;

pub use compute::compute_status;
pub use model::StatusEntry;
