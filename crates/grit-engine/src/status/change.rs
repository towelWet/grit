use super::model::StatusEntry;
use crate::objects::TreeEntry;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::collections::BTreeMap;

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

fn status(old: &TreeEntry, new: &TreeEntry) -> Option<&'static str> {
    if old.kind != new.kind {
        Some("type-changed")
    } else if old.target != new.target {
        Some("symlink-changed")
    } else if old.hash != new.hash {
        Some("modified")
    } else if old.mode != new.mode {
        Some("permission-changed")
    } else {
        None
    }
}
