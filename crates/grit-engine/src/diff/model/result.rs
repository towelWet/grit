use super::DiffEntry;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiffResult {
    pub entries: Vec<DiffEntry>,
    pub files_changed: u64,
    pub lines_added: u64,
    pub lines_deleted: u64,
}

impl DiffResult {
    pub(crate) fn from_entries(entries: Vec<DiffEntry>) -> Self {
        let lines_added = entries.iter().filter_map(|e| e.lines_added).sum();
        let lines_deleted = entries.iter().filter_map(|e| e.lines_deleted).sum();
        Self {
            files_changed: entries.len() as u64,
            entries,
            lines_added,
            lines_deleted,
        }
    }
}
