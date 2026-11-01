// include_line.rs — one item split out of generate.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn include_line(path: &Path) -> String {
    format!("include!({:?});\n", path.canonicalize().unwrap())
}
