// object_exists.rs — does a content-addressed object file exist on disk?
// Why: snapshots skip rewriting identical objects; a single stat is much cheaper than reading the object.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::{hash_to_storage_path, Hash};
use crate::store::paths::objects_dir;
use std::path::Path;

pub fn object_exists(root: &Path, hash: &Hash) -> bool {
    objects_dir(root).join(hash_to_storage_path(hash)).exists() // existence only; bytes are verify's job
}
