// read_commit.rs — decode a commit object from its JSON payload.
// Why: commits are always JSON; the format field inside them selects how their trees are read.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::Commit;
use crate::store::read::read_object;
use crate::{GritError, Result};
use std::path::Path;

pub fn read_commit(root: &Path, hash: &Hash) -> Result<Commit> {
    let data = read_object(root, hash)?;
    serde_json::from_slice(&data).map_err(GritError::Json) // malformed JSON surfaces as a typed error
}
