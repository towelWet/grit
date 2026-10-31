// execute.rs — apply a restore plan: move displaced files aside, write the target, then verify.
// Why: the order (plan, safety save, lock, move aside, write, verify) means a failure never loses data silently.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::plan::plan_restore;
use super::safety_snapshot::safety_snapshot;
use super::target_entries::target_entries;
use super::types::RestoreOptions;
use crate::store::lock::Lock;
use crate::{GritError, Result};

pub fn execute_restore(opts: &RestoreOptions) -> Result<()> {
    let plan = plan_restore(opts)?;
    if !opts.force && (!plan.overwrite.is_empty() || !plan.remove.is_empty()) {
        return Err(GritError::OverwriteProtected(format!(
            "{} overwrite, {} remove",
            plan.overwrite.len(),
            plan.remove.len()
        )));
    }
    if opts.safety_save && (!plan.overwrite.is_empty() || !plan.remove.is_empty()) {
        safety_snapshot(opts)?; // record the displaced state before anything moves
    }
    let _lock = Lock::acquire(opts.root)?; // held until the function returns
    let target = target_entries(opts.root, opts.commit_hash)?; // keeps the object ids that apply must read
    let backup = super::backup::root(opts.root);
    for path in plan.remove.iter().chain(plan.overwrite.iter()) {
        super::backup::move_aside(opts.root, path, &backup)?; // displaced content is kept, never unlinked
    }
    super::apply::entries(opts.root, &target, opts.paths)?;
    super::verify::restored(opts.root, &target.entries, opts.paths)?; // disk must match the target exactly
    let changed: Vec<String> = plan
        .create
        .iter()
        .chain(&plan.overwrite)
        .chain(&plan.remove)
        .cloned()
        .collect();
    crate::journal::append(opts.root, "restore", opts.commit_hash, "HEAD", &changed)
}
