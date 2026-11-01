// snapshot_id.rs — one item split out of support.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub fn snapshot_id(output: &str) -> String {
    output.split_whitespace().nth(2).unwrap().to_string()
}
