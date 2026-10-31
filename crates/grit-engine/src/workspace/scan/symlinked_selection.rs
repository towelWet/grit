// symlinked_selection.rs — reject a --path whose route from root crosses a symlink.
// Why: walker never follows symlinks (collect_items.rs), so `--path .claude/skills`
// where `.claude` -> an external volume walked nothing and printed "(clean)" —
// false absence. A loud error beats silently reporting nothing (2026-10-29).
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::{GritError, Result};
use std::path::Path;

pub(super) fn symlinked_selection(root: &Path, selections: Option<&[String]>) -> Result<()> {
    for selection in selections.unwrap_or_default() {
        let mut current = root.to_path_buf();
        for part in Path::new(selection).components() {
            current.push(part);                              // check each prefix: a/, a/b/, a/b/c
            let is_link = std::fs::symlink_metadata(&current)
                .map(|meta| meta.file_type().is_symlink())
                .unwrap_or(false);                           // missing path = deleted file, still valid scope
            if is_link && current != root.join(selection) {  // selecting the link itself is fine (tracked as link)
                let target = std::fs::canonicalize(&current).unwrap_or_default();
                return Err(GritError::SymlinkSelection(format!(
                    "{selection} — {} is a symlink to {}; grit does not follow symlinks. \
                     Run grit inside a repo at the target (grit init there).",
                    current.display(),
                    target.display()
                )));
            }
        }
    }
    Ok(())
}
