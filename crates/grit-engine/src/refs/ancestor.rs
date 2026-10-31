// ancestor.rs — resolve `<rev>~N` (Nth first-parent ancestor; bare `~` = `~1`).
// Why: `grit diff <save>~1 <save>` is the natural "what did this save change" call;
// before this it failed with "Save not found: <save>~1" (2026-10-29).
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::{GritError, Hash, Result};
use std::path::Path;

pub(super) fn resolve_ancestor(root: &Path, value: &str) -> Option<Result<Hash>> {
    let (base, count) = value.rsplit_once('~')?;            // no `~` → not ours, caller continues
    let steps: usize = if count.is_empty() { 1 } else { count.parse().ok()? }; // `~x` junk → not ours
    let resolved = super::resolve_revision(root, base);     // base may itself be `a~2` (chained)
    Some(resolved.and_then(|mut hash| {
        for _ in 0..steps {
            let commit = crate::store::read::read_commit(root, &hash)?;
            hash = *commit.parents.first().ok_or_else(|| {  // first parent = mainline, like git
                GritError::SaveNotFound(format!("{value} (history ends before that)"))
            })?;
        }
        Ok(hash)
    }))
}
