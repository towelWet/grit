// commit_matches.rs — one item split out of prefix.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(in super::super::super) fn commit_matches(root: &Path, prefix: &str) -> Result<Vec<Hash>> {
    let objects = objects_dir(root);
    let mut matches = Vec::new();
    for entry in walkdir::WalkDir::new(&objects) {
        let entry = entry?;
        if !entry.file_type().is_file() || crate::store::is_appledouble(entry.path()) {
            continue; // dirs + macOS "._" sidecars on exFAT are not objects
        }
        let relative = entry
            .path()
            .strip_prefix(&objects)
            .map_err(|_| bad_path())?;
        let hex = relative.to_string_lossy().replace('/', "");
        if hex.starts_with(prefix) {
            let hash = Hash::from_hex(&hex)?;
            if crate::store::read::read_commit(root, &hash).is_ok() {
                matches.push(hash);
            }
        }
    }
    Ok(matches)
}
