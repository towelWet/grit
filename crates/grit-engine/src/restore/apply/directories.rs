use crate::objects::tree::EntryKind;
use crate::objects::TreeEntry;
use crate::workspace::selected;
use crate::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn apply(
    root: &Path,
    entries: &BTreeMap<String, TreeEntry>,
    paths: Option<&[String]>,
) -> Result<()> {
    for (path, entry) in entries {
        if selected(path, paths) && entry.kind == EntryKind::Directory {
            let full = root.join(path);
            fs::create_dir_all(&full)?;
            super::super::filesystem::set_mode(&full, entry.mode)?;
        }
    }
    Ok(())
}
