// lock_error.rs — one item split out of acquire.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn lock_error(error: std::io::Error) -> GritError {
    if error.kind() == std::io::ErrorKind::AlreadyExists {
        GritError::TransactionConflict("repository lock raced".into())
    } else {
        GritError::Io(error)
    }
}
