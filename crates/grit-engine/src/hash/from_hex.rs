// from_hex.rs — parse a 64-character hex id back into a hash.
// Why: ids arrive from CLI arguments, refs, and TOML; a bad id must become a typed error, not a panic.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;
use crate::{GritError, Result};

impl Hash {
    pub fn from_hex(value: &str) -> Result<Self> {
        let mut output = [0u8; 32];
        hex::decode_to_slice(value, &mut output) // rejects wrong length and non-hex characters
            .map_err(|error| GritError::InvalidObject(format!("bad hash: {error}")))?;
        Ok(Self(output))
    }
}
