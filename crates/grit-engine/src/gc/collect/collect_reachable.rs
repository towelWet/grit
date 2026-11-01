// collect_reachable.rs — one item split out of collect.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn collect_reachable(root: &Path) -> Result<HashSet<Hash>> {
    let mut reachable = HashSet::new();
    let mut pending = roots(root)?;
    while let Some(hash) = pending.pop() {
        if !reachable.insert(hash) {
            continue;
        }
        let Ok(bytes) = crate::store::read::read_object(root, &hash) else {
            continue;
        };
        if let Ok(commit) = serde_json::from_slice::<Commit>(&bytes) {
            pending.push(commit.root_tree);
            pending.extend(commit.parents);
        } else if let Ok(tree) = serde_json::from_slice::<Tree>(&bytes) {
            pending.extend(tree.entries.values().map(|entry| entry.hash));
        }
    }
    Ok(reachable)
}
