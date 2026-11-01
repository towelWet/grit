// get_tag.rs — one item split out of tag.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn get_tag(root: &Path, tag: &str) -> Result<Hash> {
    super::super::name::validate(tag)?;
    let value = fs::read_to_string(tags_dir(root).join(tag))
        .map_err(|_| GritError::RefNotFound(tag.into()))?;
    Hash::from_hex(value.trim())
}
