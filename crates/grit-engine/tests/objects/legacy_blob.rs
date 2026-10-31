// legacy_blob.rs — an object written in the old JSON blob format must still read back exactly.
// Why: repositories are never migrated by force, so every reader has to accept the legacy encoding.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{fresh_repo, write_legacy_object};
use grit_engine::objects::Blob;
use grit_engine::store::read::read_blob;

#[test]
fn legacy_json_blob_reads_back_exactly() {
    let repo = fresh_repo();
    let payload = serde_json::to_vec(&Blob::new(b"old bytes".to_vec())).unwrap(); // exact old on-disk JSON
    let hash = write_legacy_object(repo.path(), &payload);
    let blob = read_blob(repo.path(), &hash).unwrap();
    assert_eq!(blob.data, b"old bytes"); // decoded through the legacy branch
    assert_eq!(blob.size, 9);
}
