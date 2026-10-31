// storage_path.rs — relative object path "ab/cdef…" under objects/.
// Why: a two-character fan-out directory keeps any single directory from holding every object.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;

pub fn hash_to_storage_path(hash: &Hash) -> String {
    let hex = hash.hex();
    format!("{}/{}", &hex[..2], &hex[2..]) // first byte names the fan-out directory
}
