// normalize_legacy.rs — give legacy file entries the raw-blob ids a fresh scan would produce, remembering the real object ids.
// Why: a commit written before the raw format named files by the hash of their JSON blob; without the rewrite,
// every unchanged file would look modified after upgrading, and without the memo its bytes could not be read.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::CommitEntries;
use super::read_blob;
use crate::hash::Hash;
use crate::objects::tree::EntryKind;
use crate::objects::TreeEntry;
use crate::Result;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

pub fn normalize_legacy(
    root: &Path,
    entries: BTreeMap<String, TreeEntry>,
) -> Result<CommitEntries> {
    let mut normalized = BTreeMap::new();
    let mut objects = HashMap::new(); // raw id -> legacy object id, for every rewritten file
    for (path, mut entry) in entries {
        if entry.kind != EntryKind::Directory {
            let stored = entry.hash; // the id the bytes are actually stored under
            let blob = read_blob(root, &stored)?; // decodes the legacy JSON blob
            entry.hash = Hash::of_blob(&blob.data); // the id a raw write of these bytes gets
            objects.insert(entry.hash, stored); // reads must go through this memo, not the raw id
        }
        normalized.insert(path, entry); // directories keep their constant empty-tree id
    }
    Ok(CommitEntries {
        entries: normalized,
        objects,
    })
}
