// get_branch_ref.rs — one item split out of branch.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn get_branch_ref(root: &Path, branch: &str) -> Result<Hash> {
    super::super::name::validate(branch)?;
    let value = fs::read_to_string(heads_dir(root).join(branch))
        .map_err(|_| GritError::RefNotFound(branch.into()))?;
    Hash::from_hex(value.trim())
}
