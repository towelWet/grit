// gritignore.rs — read repo-root .gritignore into exclude patterns.
// Why: projects wrote data/ here and grit never read it, so it hashed whole media libraries.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use std::fs;
use std::path::Path;

pub(super) fn gritignore(root: &Path) -> Vec<String> {
    fs::read_to_string(root.join(".gritignore"))
        .unwrap_or_default()                                  // no file = no extra rules
        .lines()
        .map(|line| line.trim().trim_start_matches('/').to_string())   // "/data" means root data, as prefixes do
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}
