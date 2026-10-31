use crate::Hash;

#[derive(Debug, Clone)]
pub struct StatusEntry {
    pub path: String,
    pub status: String,
    pub old_hash: Option<Hash>,
    pub new_hash: Option<Hash>,
}

impl StatusEntry {
    pub fn new(path: String, status: &str, old_hash: Option<Hash>, new_hash: Option<Hash>) -> Self {
        Self {
            path,
            status: status.into(),
            old_hash,
            new_hash,
        }
    }
}
