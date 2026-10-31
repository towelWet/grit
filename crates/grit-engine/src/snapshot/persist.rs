// persist.rs — write the new tree and commit objects for a snapshot.
// Why: trees are rebuilt only along changed paths, and the commit records the changed paths and the root pointer.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod build_dir;
mod direct_children;
mod persist_commit;
mod persist_tree;
mod store_tree;

pub use persist_commit::persist_commit;
pub use persist_tree::persist_tree;
