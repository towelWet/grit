// missing.rs — one item split out of verify.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn missing(path: &str) -> GritError {
    GritError::Verification(format!("missing {path}"))
}
