// as_bytes.rs — borrow the raw 32-byte digest without copying.
// Why: callers that feed another hasher or a byte map need the array, not a hex string.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;

impl Hash {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
