use crate::{Hash, TreeEntry};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiffEntry {
    pub path: String,
    pub status: String,
    pub old_hash: Option<Hash>,
    pub new_hash: Option<Hash>,
    pub old_size: Option<u64>,
    pub new_size: Option<u64>,
    pub lines_added: Option<u64>,
    pub lines_deleted: Option<u64>,
    pub patch: Option<String>,
}

impl DiffEntry {
    pub(crate) fn new(
        path: String,
        status: &str,
        old: Option<&TreeEntry>,
        new: Option<&TreeEntry>,
        old_bytes: Option<Vec<u8>>,
        new_bytes: Option<Vec<u8>>,
    ) -> Self {
        let delta = super::super::text::delta(&path, old_bytes.as_deref(), new_bytes.as_deref());
        Self {
            path,
            status: status.into(),
            old_hash: old.map(|e| e.hash),
            new_hash: new.map(|e| e.hash),
            old_size: old_bytes.as_ref().map(|b| b.len() as u64),
            new_size: new_bytes.as_ref().map(|b| b.len() as u64),
            lines_added: delta.0,
            lines_deleted: delta.1,
            patch: delta.2,
        }
    }
}
