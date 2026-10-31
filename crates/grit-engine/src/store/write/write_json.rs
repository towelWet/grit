// write_json.rs — serialize a typed object to JSON, then store it.
// Why: trees and commits are JSON; this keeps callers from repeating the serialize-then-write pair.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::store::write::write_object;
use crate::Result;
use std::path::Path;

pub fn write_json<T: serde::Serialize>(root: &Path, obj: &T) -> Result<Hash> {
    write_object(root, &serde_json::to_vec(obj)?) // serialization errors map to GritError::Json
}
