// mod.rs — machine-readable report shapes and the TOML renderer behind `--format toml`.
// Why: scripts depend on these key names, so existing keys are never renamed or removed; new keys are only added.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    pub repository_root: String,
    pub branch: String,
    pub changed_files: Vec<String>,
    pub added_files: Vec<String>,
    pub modified_files: Vec<String>,
    pub deleted_files: Vec<String>,
    pub renamed_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotReport {
    pub repository_root: String,
    pub snapshot_id: String, // the snapshot current after the command (HEAD); empty when none exists
    pub parent_snapshot_id: Option<String>,
    pub branch: String,
    pub changed_files: Vec<String>,
    pub added_files: Vec<String>,
    pub modified_files: Vec<String>,
    pub deleted_files: Vec<String>,
    pub renamed_files: Vec<String>,
    pub object_count: u64,
    pub bytes_added: u64,
    pub bytes_reused: u64,
    pub verification_status: String,
    pub transaction_id: String,
    pub saved: bool, // false when nothing changed and no commit was created
    #[serde(skip_serializing_if = "Option::is_none")] // present only when a new commit was written
    pub id: Option<String>,
    pub files: u64, // number of changed paths, the same count the human output reports
}

pub fn format_toml<T: Serialize>(v: &T) -> String {
    toml::to_string_pretty(v).unwrap_or_else(|_| "ser error".into())
}
