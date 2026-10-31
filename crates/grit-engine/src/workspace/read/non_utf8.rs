// non_utf8.rs — the error for a path that cannot be stored as UTF-8.
// Why: tree entries are keyed by UTF-8 strings, so such a path must fail loudly rather than be mangled.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::GritError;

pub(super) fn non_utf8() -> GritError {
    GritError::InvalidObject("non-UTF-8 repository path".into()) // names the problem; the path is in the scan error
}
