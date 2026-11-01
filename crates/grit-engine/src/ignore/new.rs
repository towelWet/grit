// new.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl IgnoreRules {
    pub fn new(patterns: &[String]) -> Self {
        let mut b = GlobSetBuilder::new();
        for p in patterns {
            if let Ok(g) = Glob::new(p) {
                b.add(g);
            }
        }
        Self {
            inner: b.build().unwrap_or(GlobSet::empty()),
            prefixes: patterns
                .iter()
                .map(|p| p.trim_end_matches('/').to_string())
                .filter(|p| !p.is_empty() && !p.contains(['*', '?', '[', '{']))
                .collect(),
        }
    }
}
