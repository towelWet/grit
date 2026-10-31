use crate::hash::Hash;
use crate::store::paths::journal_dir;
use crate::Result;
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::path::Path;
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
pub fn read_journal(root: &Path, limit: usize) -> Result<Vec<serde_json::Value>> {
    let dir = journal_dir(root);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut entries = vec![];
    let mut files: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
    files.sort_by_key(|e| e.file_name());
    for f in files {
        let c = fs::read_to_string(f.path())?;
        for l in c.lines() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                entries.push(v);
            }
        }
    }
    entries.reverse();
    entries.truncate(limit);
    Ok(entries)
}
