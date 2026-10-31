// mod.rs — restore module: plan, execute, and verify a checkout of a saved state.
// Why: restores are destructive, so planning and execution are separate, reviewable steps.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod apply;
mod backup;
mod current;
mod execute;
mod filesystem;
mod order;
mod plan;
mod safety_snapshot;
mod target_entries;
mod types;
mod verify;

pub use execute::execute_restore;
pub use plan::plan_restore;
pub use types::{RestoreOptions, RestorePlan};
