// mod.rs — fixtures shared by the object-format tests (one function per file alongside this file).
// Why: each test needs a fresh repository, a way to write old-format objects, deterministic bytes, and snapshot helpers.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod child_hash;
mod fresh_repo;
mod pseudo_random_bytes;
mod save_snapshot;
mod write_legacy_object;

pub use child_hash::child_hash;
pub use fresh_repo::fresh_repo;
pub use pseudo_random_bytes::pseudo_random_bytes;
pub use save_snapshot::save_snapshot;
pub use write_legacy_object::write_legacy_object;
