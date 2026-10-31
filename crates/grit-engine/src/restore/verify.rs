use crate::objects::TreeEntry;
use crate::workspace::selected;
use crate::{GritError, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn restored(
    root: &Path,
    target: &BTreeMap<String, TreeEntry>,
    paths: Option<&[String]>,
) -> Result<()> {
    let current = super::current::scan(root, paths)?;
    for (path, expected) in target.iter().filter(|(path, _)| selected(path, paths)) {
        let actual = current
            .get(path)
            .ok_or_else(|| missing(path))?
            .tree_entry(path)?;
        if actual != *expected {
            return Err(GritError::Verification(path.clone()));
        }
    }
    if paths.is_none() {
        let actual: BTreeSet<_> = current.keys().collect();
        let expected: BTreeSet<_> = target.keys().collect();
        if actual != expected {
            return Err(GritError::Verification("working tree path set".into()));
        }
    }
    Ok(())
}

fn missing(path: &str) -> GritError {
    GritError::Verification(format!("missing {path}"))
}
