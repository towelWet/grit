// list_branches.rs — one item split out of branch.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn list_branches(root: &Path) -> Result<Vec<String>> {
    list_names(&heads_dir(root))
}
