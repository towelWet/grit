// hash_from_path.rs — one item split out of collect.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn hash_from_path(objects: &Path, path: &Path) -> Result<Hash> {
    let relative = path
        .strip_prefix(objects)
        .map_err(|_| GritError::Internal("object path".into()))?;
    Hash::from_hex(&relative.to_string_lossy().replace('/', ""))
}
