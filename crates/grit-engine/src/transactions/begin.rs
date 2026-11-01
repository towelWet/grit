// begin.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

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
