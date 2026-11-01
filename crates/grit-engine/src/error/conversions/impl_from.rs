// impl_from.rs — one item split out of conversions.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl From<&str> for GritError {
    fn from(value: &str) -> Self {
        GritError::Internal(value.into())
    }
}
