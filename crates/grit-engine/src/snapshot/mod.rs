// mod.rs — snapshot pipeline: scan, store objects, classify changes, then commit.
// Why: each stage is its own module so the ordering (store before commit) is visible in one place.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod base_entries;
mod collect_changed;
mod dirty_dirs;
pub(crate) mod classify;
mod create;
mod load;
mod no_change_result;
mod object_stats_add;
mod parent_root;
mod persist;
mod prepare;
mod scan_scope;
mod apply_raced;
mod store_changed;
mod store_entry;
mod store_file;
mod store_kept;
mod types;

pub use create::create_snapshot;
pub use types::{SnapshotOptions, SnapshotResult};
