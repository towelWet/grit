// selected.rs — one item split out of path.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub fn selected(path: &str, selections: Option<&[String]>) -> bool {
    selections.is_none_or(|items| {
        items.iter().any(|item| {
            path == item
                || path.starts_with(&format!("{item}/"))
                || item.starts_with(&format!("{path}/"))
        })
    })
}
