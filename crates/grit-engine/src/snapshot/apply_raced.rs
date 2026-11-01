// apply_raced.rs — update tree map with files that changed or vanished after scan.
// Why: tree must name objects that actually exist; old scan id would point at nothing.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::TreeEntry;
use crate::Hash;
use std::collections::BTreeMap;

pub type Raced = (String, Option<Hash>); // path, id of bytes stored; None = file vanished

pub(super) fn apply_raced(new: &mut BTreeMap<String, TreeEntry>, raced: Vec<Raced>) {
    for (path, stored) in raced {
        match stored {
            Some(hash) => {
                if let Some(entry) = new.get_mut(&path) {
                    entry.hash = hash; // stat cache still holds old stat, so next scan rehashes it
                }
            }
            None => {
                new.remove(&path); // absent from this snapshot, same as deleted before scan
            }
        }
    }
}
