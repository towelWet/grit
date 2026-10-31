// short.rs — 12-character prefix of the hex id for human output.
// Why: users type and read short ids; six bytes stay unique enough for a local repository.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;

impl Hash {
    pub fn short(&self) -> String {
        hex::encode(&self.0[..6]) // six bytes = twelve hex characters
    }
}
