use crate::objects::{Commit, Tree};
use crate::{GritError, Hash, Result};
use std::collections::HashSet;
use std::path::Path;

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

fn roots(root: &Path) -> Result<Vec<Hash>> {
    let branches = crate::refs::list_branches(root)?
        .into_iter()
        .filter_map(|name| crate::refs::get_branch_ref(root, &name).ok());
    let tags = crate::refs::list_tags(root)?
        .into_iter()
        .filter_map(|name| crate::refs::get_tag(root, &name).ok());
    Ok(branches.chain(tags).collect())
}

pub fn hash_from_path(objects: &Path, path: &Path) -> Result<Hash> {
    let relative = path
        .strip_prefix(objects)
        .map_err(|_| GritError::Internal("object path".into()))?;
    Hash::from_hex(&relative.to_string_lossy().replace('/', ""))
}
