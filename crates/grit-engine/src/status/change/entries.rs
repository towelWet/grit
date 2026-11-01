// entries.rs — one item split out of change.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn entries(
    old: &BTreeMap<String, TreeEntry>,
    current: &BTreeMap<String, WorkspaceEntry>,
) -> Result<Vec<StatusEntry>> {
    let mut result = Vec::new();
    for (path, before) in old {
        let Some(now) = current.get(path) else {
            continue;
        };
        let after = now.tree_entry(path)?;
        if let Some(status) = status(before, &after) {
            result.push(StatusEntry::new(
                path.clone(),
                status,
                Some(before.hash),
                Some(after.hash),
            ));
        }
    }
    Ok(result)
}
