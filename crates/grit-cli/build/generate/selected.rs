// selected.rs — one item split out of generate.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn selected(directory: &str, keep: impl Fn(&str) -> bool) -> Vec<PathBuf> {
    println!("cargo:rerun-if-changed={directory}");
    let mut files = fs::read_dir(directory)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter(|path| keep(path.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<_>>();
    files.sort();
    files
}
