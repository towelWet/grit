// objects.rs — integration tests for stored object formats: legacy readability and raw-blob size.
// Why: repositories written by older builds must keep reading, and raw blobs must not inflate the store.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
#[path = "objects/nested_tree.rs"]
mod nested_tree; // clean subtrees keep their ids and commits record only changed paths
#[path = "objects/legacy_blob.rs"]
mod legacy_blob; // JSON blob written by the old format must still decode
#[path = "objects/legacy_commit.rs"]
mod legacy_commit; // old commit without `format` must normalize to raw ids
#[path = "objects/raw_size.rs"]
mod raw_size; // 1 MB of random bytes must store at about 1 MB, not 1.4 MB
#[path = "objects/support/mod.rs"]
mod support; // shared fixtures: repository, legacy writer, pseudo-random bytes
