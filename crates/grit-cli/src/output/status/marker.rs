// marker.rs — one item split out of status.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub(super) fn marker(status: &str) -> &str {
    match status {
        "modified" => "M",
        "added" => "A",
        "deleted" => "D",
        "renamed" => "R",
        "type-changed" => "T",
        "permission-changed" => "P",
        "symlink-changed" => "L",
        _ => "?",
    }
}
