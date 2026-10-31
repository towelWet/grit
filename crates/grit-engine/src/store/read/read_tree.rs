// read_tree.rs — decode a tree object from its JSON payload.
// Why: trees are JSON maps from path (format 1) or child name (format 2) to entries.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::Tree;
use crate::store::read::read_object;
use crate::{GritError, Result};
use std::path::Path;

pub fn read_tree(root: &Path, hash: &Hash) -> Result<Tree> {
    let data = read_object(root, hash)?;
    serde_json::from_slice(&data).map_err(GritError::Json) // same JSON error mapping as commits
}
