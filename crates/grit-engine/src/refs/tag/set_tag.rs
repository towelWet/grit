// set_tag.rs — one item split out of tag.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn set_tag(root: &Path, tag: &str, hash: &Hash) -> Result<()> {
    super::super::name::validate(tag)?;
    let path = tags_dir(root).join(tag);
    if path.exists() {
        return Err(GritError::TagExists(tag.into()));
    }
    crate::store::write::atomic_write(&path, hash.hex().as_bytes())
}
