// get_current_branch.rs — one item split out of head.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn get_current_branch(root: &Path) -> Result<String> {
    get_head(root)?
        .strip_prefix("ref: refs/heads/")
        .map(|branch| branch.trim().to_string())
        .ok_or_else(|| GritError::Internal("detached".into()))
}
