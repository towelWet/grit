// legacy_commit.rs — a commit written before the `format` field must normalize to raw-blob ids.
// Why: without normalization every unchanged file would look modified right after upgrading.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{fresh_repo, write_legacy_object};
use grit_engine::hash::Hash;
use grit_engine::objects::Blob;
use grit_engine::store::read::{commit_entries, read_blob};

#[test]
fn legacy_commit_entries_get_raw_blob_ids() {
    let repo = fresh_repo();
    let file = serde_json::to_vec(&Blob::new(b"old".to_vec())).unwrap(); // legacy JSON blob
    let file_hash = write_legacy_object(repo.path(), &file);
    // Old tree: flat map keyed by full path, entries carry the legacy blob hash.
    let tree = serde_json::json!({"entries": {"a.txt": {
        "name": "a.txt", "kind": "File", "hash": file_hash.hex(), "mode": 33188, "target": null
    }}});
    let tree_hash = write_legacy_object(repo.path(), &serde_json::to_vec(&tree).unwrap());
    // Old commit: no `format` key, so it deserializes as format 0 (legacy).
    let commit = serde_json::json!({
        "root_tree": tree_hash.hex(), "parents": [], "author": "t",
        "message": "m", "timestamp": "2026-01-01T00:00:00Z"
    });
    let commit_hash = write_legacy_object(repo.path(), &serde_json::to_vec(&commit).unwrap());
    let commit = commit_entries(repo.path(), &commit_hash).unwrap();
    let entry = &commit.entries["a.txt"];
    assert_eq!(entry.hash, Hash::of_blob(b"old")); // comparisons use the raw-blob id
    let stored = commit.object_id(entry);
    assert_eq!(stored, file_hash); // ...but the bytes still live under the legacy object id
    assert_eq!(read_blob(repo.path(), &stored).unwrap().data, b"old"); // restore and diff read them here
}
