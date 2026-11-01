// get_head.rs — one item split out of head.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn get_head(root: &Path) -> Result<String> {
    let path = head_path(root);
    if !path.exists() {
        return Err(GritError::RefNotFound("HEAD".into()));
    }
    Ok(fs::read_to_string(path)?.trim().to_string())
}
