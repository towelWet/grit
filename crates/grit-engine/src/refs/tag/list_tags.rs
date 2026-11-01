// list_tags.rs — one item split out of tag.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn list_tags(root: &Path) -> Result<Vec<String>> {
    super::super::branch::list_names(&tags_dir(root))
}
