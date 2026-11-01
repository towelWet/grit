// append.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn append(root: &Path, op: &str, hash: &Hash, refname: &str, changed: &[String]) -> Result<()> {
    let d = journal_dir(root);
    fs::create_dir_all(&d)?;
    let p = d.join(format!("{}.jsonl", Utc::now().format("%Y%m%d")));
    let mut f = fs::OpenOptions::new().create(true).append(true).open(&p)?;
    writeln!(
        f,
        "{}",
        serde_json::json!({"op":op,"hash":hash.hex(),"ref":refname,"timestamp":Utc::now().to_rfc3339(),"changed":changed})
    )?;
    Ok(())
}
