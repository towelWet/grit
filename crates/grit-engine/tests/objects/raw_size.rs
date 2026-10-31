// raw_size.rs — a 1 MB random file must be stored at about its own size and read back byte-exact.
// Why: the JSON byte-array encoding stored it at 1.47 MB (+41%); raw storage must stay within a few KB.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{fresh_repo, pseudo_random_bytes};
use grit_engine::hash::{hash_to_storage_path, Hash};
use grit_engine::store::paths::objects_dir;
use grit_engine::store::read::read_blob;
use grit_engine::store::write::write_blob;

#[test]
fn one_megabyte_random_blob_stores_near_one_megabyte() {
    let repo = fresh_repo();
    let data = pseudo_random_bytes(1 << 20, 9389); // random: compression cannot hide per-byte overhead
    let (hash, size) = write_blob(repo.path(), &data[..]).unwrap();
    assert_eq!(size, data.len() as u64);
    assert_eq!(hash, Hash::of_blob(&data)); // id is the domain-tagged hash of the raw bytes
    let path = objects_dir(repo.path()).join(hash_to_storage_path(&hash));
    let stored = std::fs::metadata(path).unwrap().len();
    assert!(
        stored < (1 << 20) + 4096,
        "stored {stored} bytes for {} input bytes",
        data.len()
    ); // zstd framing on random data adds only a few bytes
    assert_eq!(read_blob(repo.path(), &hash).unwrap().data, data); // round trip is exact
}
