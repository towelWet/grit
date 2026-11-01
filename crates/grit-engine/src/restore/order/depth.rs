// depth.rs — one item split out of order.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub(super) fn depth(path: &str) -> usize {
    path.matches('/').count()
}
