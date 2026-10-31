use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    inner: GlobSet,
    prefixes: Vec<String>,
}
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
    pub fn empty() -> Self {
        Self {
            inner: GlobSet::empty(),
            prefixes: Vec::new(),
        }
    }
    pub fn is_ignored(&self, path: &Path) -> bool {
        if self.inner.is_match(path) {
            return true;
        }
        let value = path.to_string_lossy();
        self.prefixes
            .iter()
            .any(|prefix| value == *prefix || value.starts_with(&format!("{prefix}/")))
    }
}
