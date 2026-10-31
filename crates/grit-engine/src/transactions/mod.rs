use crate::store::paths::transactions_dir;
use crate::store::write::atomic_write;
use crate::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub op: String,
    pub state: String,
    pub snapshot_before: Option<String>,
    pub snapshot_after: Option<String>,
    pub created: String,
}
pub fn begin(root: &Path, op: &str) -> Result<String> {
    let d = transactions_dir(root);
    fs::create_dir_all(&d)?;
    let id = format!("{}-{}", Utc::now().format("%Y%m%d%H%M%S%f"), op);
    let tx = Transaction {
        id: id.clone(),
        op: op.into(),
        state: "pending".into(),
        snapshot_before: None,
        snapshot_after: None,
        created: Utc::now().to_rfc3339(),
    };
    let bytes = toml::to_string_pretty(&tx)?;
    atomic_write(&d.join(format!("{id}.toml")), bytes.as_bytes())?;
    Ok(id)
}
pub fn commit(root: &Path, id: &str) -> Result<()> {
    upd(root, id, "committed")
}
pub fn rollback(root: &Path, id: &str) -> Result<()> {
    upd(root, id, "rolled_back")
}
fn upd(root: &Path, id: &str, state: &str) -> Result<()> {
    let p = transactions_dir(root).join(format!("{id}.toml"));
    let mut tx: Transaction = toml::from_str(&fs::read_to_string(&p)?)?;
    tx.state = state.into();
    let bytes = toml::to_string_pretty(&tx)?;
    atomic_write(&p, bytes.as_bytes())?;
    Ok(())
}
