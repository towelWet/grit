// set_branch_ref.rs — one item split out of branch.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn set_branch_ref(root: &Path, branch: &str, hash: &Hash) -> Result<()> {
    super::super::name::validate(branch)?;
    crate::store::write::atomic_write(&heads_dir(root).join(branch), hash.hex().as_bytes())
}
