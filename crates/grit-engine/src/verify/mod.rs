mod model;
mod object;

pub use model::VerifyResult;

use crate::gc::collect_reachable;
use crate::store::paths::objects_dir;
use crate::Result;
use std::path::Path;

pub fn verify(root: &Path) -> Result<VerifyResult> {
    let objects = objects_dir(root);
    let mut result = VerifyResult::default();
    if !objects.exists() {
        return Ok(result);
    }
    for entry in walkdir::WalkDir::new(&objects) {
        let entry = entry?;
        if !entry.file_type().is_file() || crate::store::is_appledouble(entry.path()) {
            continue; // dirs + macOS "._" sidecars on exFAT are not objects
        }
        result.total_objects += 1;
        match object::verify(&objects, entry.path()) {
            Ok(()) => result.valid += 1,
            Err(error) => {
                result.corrupt += 1;
                result.errors.push(error.to_string());
            }
        }
    }
    result.reachable = collect_reachable(root)?.len() as u64;
    result.unreachable = result.total_objects.saturating_sub(result.reachable);
    Ok(result)
}
