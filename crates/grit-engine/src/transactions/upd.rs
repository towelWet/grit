// upd.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn upd(root: &Path, id: &str, state: &str) -> Result<()> {
    let p = transactions_dir(root).join(format!("{id}.toml"));
    let mut tx: Transaction = toml::from_str(&fs::read_to_string(&p)?)?;
    tx.state = state.into();
    let bytes = toml::to_string_pretty(&tx)?;
    atomic_write(&p, bytes.as_bytes())?;
    Ok(())
}
