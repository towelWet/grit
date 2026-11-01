// resolve_revision.rs — one item split out of resolve.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn resolve_revision(root: &Path, value: &str) -> Result<Hash> {
    if let Ok(hash) = resolve_ref(root, value) {
        return Ok(hash);
    }
    if let Some(result) = super::super::ancestor::resolve_ancestor(root, value) {
        return result;                                       // `<rev>~N` form (refs/ancestor.rs)
    }
    if value.len() == 64 {
        let hash = Hash::from_hex(value)?;
        crate::store::read::read_commit(root, &hash)?;
        return Ok(hash);
    }
    if value.len() < 6 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(GritError::SaveNotFound(value.into()));
    }
    let matches = super::super::prefix::commit_matches(root, value)?;
    match matches.as_slice() {
        [hash] => Ok(*hash),
        [] => Err(GritError::SaveNotFound(value.into())),
        _ => Err(GritError::InvalidObject(format!(
            "ambiguous revision: {value}"
        ))),
    }
}
