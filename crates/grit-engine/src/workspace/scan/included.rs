// included.rs — decide whether the walker should descend into or report one entry.
// Why: ignored paths and paths outside --path scope must be pruned before any metadata is read.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::ignore::IgnoreRules;
use crate::workspace::selected;
use std::path::Path;
use walkdir::DirEntry;

pub(super) fn included(
    root: &Path,
    ignore: &IgnoreRules,
    selections: Option<&[String]>,
    entry: &DirEntry,
) -> bool {
    let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
    let path = relative.to_string_lossy();
    !ignore.is_ignored(relative) && (path.is_empty() || selected(&path, selections)) // root is always walked
}
