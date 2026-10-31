// to_hex.rs — full 64-character lowercase hex form of a hash.
// Why: hex names object files, refs, and TOML/JSON reports, so one canonical rendering is required.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;

impl Hash {
    pub fn hex(&self) -> String {
        hex::encode(self.0)
    }
}
