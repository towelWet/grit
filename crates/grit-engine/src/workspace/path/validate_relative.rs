// validate_relative.rs — one item split out of path.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn validate_relative(path: &str) -> Result<()> {
    let value = Path::new(path);
    let invalid = value.as_os_str().is_empty()
        || value.is_absolute()
        || value
            .components()
            .any(|part| !matches!(part, Component::Normal(_)));
    if invalid {
        return Err(GritError::PathTraversal(path.into()));
    }
    Ok(())
}
