// run.rs — one item split out of support.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grit"))
        .args(args)
        .arg("--repo")
        .arg(root)
        .output()
        .unwrap()
}
