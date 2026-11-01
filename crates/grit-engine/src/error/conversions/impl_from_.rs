// impl_from_.rs — one item split out of conversions.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl From<walkdir::Error> for GritError {
    fn from(error: walkdir::Error) -> Self {
        GritError::Io(std::io::Error::other(error.to_string()))
    }
}
