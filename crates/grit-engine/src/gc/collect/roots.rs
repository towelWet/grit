// roots.rs — one item split out of collect.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn roots(root: &Path) -> Result<Vec<Hash>> {
    let branches = crate::refs::list_branches(root)?
        .into_iter()
        .filter_map(|name| crate::refs::get_branch_ref(root, &name).ok());
    let tags = crate::refs::list_tags(root)?
        .into_iter()
        .filter_map(|name| crate::refs::get_tag(root, &name).ok());
    Ok(branches.chain(tags).collect())
}
