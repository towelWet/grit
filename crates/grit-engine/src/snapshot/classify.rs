use crate::objects::TreeEntry;
use std::collections::{BTreeMap, BTreeSet};

pub type Changes = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);

pub fn classify(old: &BTreeMap<String, TreeEntry>, new: &BTreeMap<String, TreeEntry>) -> Changes {
    let mut added: Vec<_> = new
        .keys()
        .filter(|p| !old.contains_key(*p))
        .cloned()
        .collect();
    let mut deleted: Vec<_> = old
        .keys()
        .filter(|p| !new.contains_key(*p))
        .cloned()
        .collect();
    let modified = old
        .iter()
        .filter(|(path, entry)| new.get(*path).is_some_and(|new| new != *entry))
        .map(|(path, _)| path.clone())
        .collect();
    let (mut used_added, mut used_deleted) = (BTreeSet::new(), BTreeSet::new());
    let mut renamed = Vec::new();
    for old_path in &deleted {
        let old_entry = &old[old_path];
        let found = added.iter().find(|new_path| {
            !used_added.contains(*new_path) && same_content(old_entry, &new[*new_path])
        });
        if let Some(new_path) = found {
            used_deleted.insert(old_path.clone());
            used_added.insert(new_path.clone());
            renamed.push(format!("{old_path} -> {new_path}"));
        }
    }
    added.retain(|path| !used_added.contains(path));
    deleted.retain(|path| !used_deleted.contains(path));
    (added, modified, deleted, renamed)
}

pub fn same_content(left: &TreeEntry, right: &TreeEntry) -> bool {
    left.kind == right.kind
        && left.hash == right.hash
        && left.mode == right.mode
        && left.target == right.target
}
