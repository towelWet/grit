// direct_children.rs — the flat entries that sit exactly one level below a directory.
// Why: a directory's tree lists only its own children; deeper entries belong to subtrees and are reached recursively.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::TreeEntry;
use std::collections::BTreeMap;

pub(super) fn direct_children<'a>(
    flat: &'a BTreeMap<String, TreeEntry>,
    prefix: &str,
) -> Vec<(&'a String, &'a TreeEntry)> {
    let start = if prefix.is_empty() {
        String::new() // the root's children have no prefix to strip
    } else {
        format!("{prefix}/") // every descendant of prefix starts with this
    };
    flat.range(start.clone()..) // sorted keys: descendants form one contiguous run
        .take_while(|(path, _)| path.starts_with(&start)) // stop at the first key outside the subtree
        .filter(|(path, _)| !path[start.len()..].contains('/')) // keep only one level below prefix
        .collect()
}
