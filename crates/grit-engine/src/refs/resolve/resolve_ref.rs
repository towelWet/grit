// resolve_ref.rs — one item split out of resolve.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn resolve_ref(root: &Path, name: &str) -> Result<Hash> {
    match name {
        "HEAD" => super::super::get_head_hash(root),
        value if value.starts_with("refs/heads/") => super::super::get_branch_ref(root, &value[11..]),
        value if value.starts_with("refs/tags/") => super::super::get_tag(root, &value[10..]),
        _ => super::super::get_branch_ref(root, name).or_else(|_| super::super::get_tag(root, name)),
    }
}
