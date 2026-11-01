// bad_path.rs — one item split out of sweep.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn bad_path() -> GritError {
    GritError::Internal("object path".into())
}
