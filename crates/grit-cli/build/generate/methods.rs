// methods.rs — one item split out of generate.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn methods(directory: &str, output: &str, keep: impl Fn(&str) -> bool) {
    let files = selected(directory, keep);
    let body = files
        .iter()
        .map(|path| fs::read_to_string(path).unwrap())
        .collect::<String>();
    write(output, format!("impl GritArgs {{\n{body}}}\n"));
}
