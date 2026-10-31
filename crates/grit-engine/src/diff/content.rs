// content.rs — the stored bytes of one entry, read from whichever object actually holds them.
// Why: legacy entries carry comparable ids, but their bytes live under the legacy object ids.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::tree::EntryKind;
use crate::objects::TreeEntry;
use crate::store::read::{read_blob, CommitEntries};
use crate::Result;
use std::path::Path;

pub fn stored(root: &Path, entry: &TreeEntry, objects: &CommitEntries) -> Result<Option<Vec<u8>>> {
    if entry.kind == EntryKind::Directory {
        return Ok(None); // directories have no bytes
    }
    Ok(Some(read_blob(root, &objects.object_id(entry))?.data)) // legacy-aware object lookup
}
