// restored.rs — one item split out of verify.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn restored(
    root: &Path,
    target: &BTreeMap<String, TreeEntry>,
    paths: Option<&[String]>,
) -> Result<()> {
    let current = super::super::current::scan(root, paths)?;
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
