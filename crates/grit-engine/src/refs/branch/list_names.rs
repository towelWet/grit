// list_names.rs — one item split out of branch.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(in super::super::super) fn list_names(directory: &Path) -> Result<Vec<String>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut names: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().to_string_lossy().to_string())
        })
        .collect();
    names.sort();
    Ok(names)
}
