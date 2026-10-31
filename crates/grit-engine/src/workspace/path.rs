use crate::{GritError, Result};
use std::path::{Component, Path};

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

pub fn selected(path: &str, selections: Option<&[String]>) -> bool {
    selections.is_none_or(|items| {
        items.iter().any(|item| {
            path == item
                || path.starts_with(&format!("{item}/"))
                || item.starts_with(&format!("{path}/"))
        })
    })
}
