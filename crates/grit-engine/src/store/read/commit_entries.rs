// commit_entries.rs — a commit's flat entries for comparison, plus the object ids that hold their bytes.
// Why: legacy entries are compared by raw-content ids, yet their bytes still live under the legacy object ids.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod object_id;

use super::{flatten_tree, normalize_legacy, read_commit, read_tree};
use crate::hash::Hash;
use crate::objects::commit::{FORMAT_LEGACY, FORMAT_NESTED};
use crate::objects::TreeEntry;
use crate::Result;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

pub struct CommitEntries {
    pub entries: BTreeMap<String, TreeEntry>, // ids comparable with a fresh workspace scan
    pub objects: HashMap<Hash, Hash>,         // comparable id -> stored object id, only where they differ (legacy)
}

pub fn commit_entries(root: &Path, hash: &Hash) -> Result<CommitEntries> {
    let commit = read_commit(root, hash)?;
    match commit.format {
        FORMAT_LEGACY => {
            let entries = read_tree(root, &commit.root_tree)?.entries;
            normalize_legacy::normalize_legacy(root, entries) // rehash legacy JSON blobs, keep their ids
        }
        FORMAT_NESTED => Ok(CommitEntries {
            entries: flatten_tree::flatten_tree(root, &commit.root_tree)?, // expand per-directory trees
            objects: HashMap::new(), // nested and raw commits store bytes under their comparable ids
        }),
        _ => Ok(CommitEntries {
            entries: read_tree(root, &commit.root_tree)?.entries, // raw flat trees already carry scan ids
            objects: HashMap::new(),
        }),
    }
}
