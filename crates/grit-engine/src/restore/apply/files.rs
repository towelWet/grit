// files.rs — write every selected regular file and symlink of the target state to disk.
// Why: file bytes are read from the object that holds them (the legacy object for old commits), then written atomically.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::tree::EntryKind;
use crate::store::read::{read_blob, CommitEntries};
use crate::workspace::{selected, validate_relative};
use crate::Result;
use std::fs;
use std::path::Path;

pub(super) fn apply(root: &Path, target: &CommitEntries, paths: Option<&[String]>) -> Result<()> {
    for (path, entry) in &target.entries {
        if !selected(path, paths) || entry.kind == EntryKind::Directory {
            continue; // directories were created by the directory pass
        }
        validate_relative(path)?; // refuse traversal before any write
        let full = root.join(path);
        fs::create_dir_all(full.parent().unwrap())?;
        if entry.kind == EntryKind::File {
            let data = read_blob(root, &target.object_id(entry))?.data; // stored object, legacy or raw
            super::super::filesystem::atomic_file(&full, &data)?;
            super::super::filesystem::set_mode(&full, entry.mode)?;
        } else {
            super::super::filesystem::create_symlink(
                &full,
                entry.target.as_deref().unwrap_or_default(),
            )?;
        }
    }
    Ok(())
}
