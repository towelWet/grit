use crate::{GritError, Result};

pub fn validate(name: &str) -> Result<()> {
    let bad_char = name
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')));
    if name.is_empty()
        || name.starts_with('.')
        || name.ends_with('.')
        || name.contains("..")
        || bad_char
    {
        return Err(GritError::PathTraversal(name.into()));
    }
    Ok(())
}
