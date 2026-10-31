// read.rs — object readers: raw bytes, typed commits and trees, blobs, and existence checks.
// Why: each reader decodes one kind of object; splitting them keeps every decoder small and reviewable.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod commit_entries;
mod flatten_tree;
mod normalize_legacy;
mod object_exists;
mod read_blob;
mod read_commit;
mod read_object;
mod read_tree;

pub use commit_entries::{commit_entries, CommitEntries};
pub use flatten_tree::flatten_tree;
pub use object_exists::object_exists;
pub use read_blob::read_blob;
pub use read_commit::read_commit;
pub use read_object::read_object;
pub use read_tree::read_tree;
