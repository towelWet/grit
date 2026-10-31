mod collect;
mod model;
mod sweep;

pub(crate) use collect::collect_reachable;
pub use model::GcResult;

use crate::store::lock::Lock;
use crate::store::paths::tmp_dir;
use crate::{GritError, Result};
use chrono::Utc;
use std::path::Path;

pub fn gc(root: &Path) -> Result<GcResult> {
    let _lock = Lock::acquire(root)?;
    let verification = crate::verify::verify(root)?;
    if verification.corrupt > 0 {
        return Err(GritError::Gc(format!(
            "{} corrupt objects",
            verification.corrupt
        )));
    }
    let reachable = collect_reachable(root)?;
    let recovery = tmp_dir(root)
        .join("gc-trash")
        .join(Utc::now().format("%Y%m%d%H%M%S%f").to_string());
    sweep::unreachable(root, &reachable, &recovery)
}
