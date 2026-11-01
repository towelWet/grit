// read_journal.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn read_journal(root: &Path, limit: usize) -> Result<Vec<serde_json::Value>> {
    let dir = journal_dir(root);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut entries = vec![];
    let mut files: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
    files.sort_by_key(|e| e.file_name());
    for f in files {
        let c = fs::read_to_string(f.path())?;
        for l in c.lines() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                entries.push(v);
            }
        }
    }
    entries.reverse();
    entries.truncate(limit);
    Ok(entries)
}
