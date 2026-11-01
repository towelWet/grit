// includes.rs — one item split out of generate.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn includes(directory: &str, output: &str, keep: impl Fn(&str) -> bool) {
    let files = selected(directory, keep);
    let content = files
        .iter()
        .map(PathBuf::as_path)
        .map(include_line)
        .collect::<String>();
    write(output, content);
}
