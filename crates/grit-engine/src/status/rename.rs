use super::model::StatusEntry;
use crate::objects::TreeEntry;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::collections::BTreeMap;

pub fn append_path_changes(
    result: &mut Vec<StatusEntry>,
    old: &BTreeMap<String, TreeEntry>,
    current: &BTreeMap<String, WorkspaceEntry>,
) -> Result<()> {
    let new = current
        .iter()
        .map(|(path, entry)| Ok((path.clone(), entry.tree_entry(path)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let (added, _, deleted, renamed) = crate::snapshot::classify::classify(old, &new);
    for path in added {
        result.push(StatusEntry::new(
            path.clone(),
            "added",
            None,
            Some(new[&path].hash),
        ));
    }
    for path in deleted {
        result.push(StatusEntry::new(
            path.clone(),
            "deleted",
            Some(old[&path].hash),
            None,
        ));
    }
    for path in renamed {
        let (before, after) = path.split_once(" -> ").unwrap();
        let hashes = (old[before].hash, new[after].hash);
        result.push(StatusEntry::new(
            path,
            "renamed",
            Some(hashes.0),
            Some(hashes.1),
        ));
    }
    Ok(())
}
