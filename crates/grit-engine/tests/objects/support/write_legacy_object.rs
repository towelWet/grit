// write_legacy_object.rs — store a payload the way the old writer did, for compatibility tests.
// Why: legacy reads must be tested against the exact on-disk layout old binaries produced.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use grit_engine::hash::hash_to_storage_path;
use grit_engine::store::paths::objects_dir;
use grit_engine::Hash;
use std::fs;
use std::path::Path;

pub fn write_legacy_object(root: &Path, payload: &[u8]) -> Hash {
    // Same layout the old writer used: zstd frame of the payload, named by the hash of that payload.
    let hash = Hash::of(payload);
    let path = objects_dir(root).join(hash_to_storage_path(&hash));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, zstd::encode_all(payload, 3).unwrap()).unwrap();
    hash
}
