// write.rs — one item split out of generate.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn write(output: &str, content: String) {
    let root = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fs::write(root.join(output), content).unwrap();
}
