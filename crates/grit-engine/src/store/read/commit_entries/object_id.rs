// object_id.rs — the id under which an entry's bytes are stored, given a comparable entry.
// Why: reads must use the object that really exists; for legacy entries that is not the comparable id.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::CommitEntries;
use crate::hash::Hash;
use crate::objects::TreeEntry;

impl CommitEntries {
    pub fn object_id(&self, entry: &TreeEntry) -> Hash {
        self.objects.get(&entry.hash).copied().unwrap_or(entry.hash) // raw entries are stored under their own id
    }
}
