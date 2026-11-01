// check_entry.rs — count one object file: trusted if untouched since proven, else reread.
// Why: keeps verify's walk readable and under line cap; all counting rules live here.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::{object, seen_key::seen_key, VerifyResult};
use crate::Result;
use std::collections::BTreeSet;
use std::path::Path;
use walkdir::DirEntry;

pub(super) fn check_entry(
    objects: &Path,
    entry: &DirEntry,
    seen: &BTreeSet<String>,
    proven: &mut BTreeSet<String>,
    result: &mut VerifyResult,
) -> Result<()> {
    result.total_objects += 1;
    let key = seen_key(objects, entry)?;
    if seen.contains(&key) {
        result.valid += 1;
        result.cached += 1; // reported, so nobody mistakes this run for a full reread
        proven.insert(key);
        return Ok(());
    }
    match object::verify(objects, entry.path()) {
        Ok(()) => {
            result.valid += 1;
            proven.insert(key);
        }
        Err(error) => {
            result.corrupt += 1;
            result.errors.push(error.to_string());
        }
    }
    Ok(())
}
