// plan.rs — classify every target path as create, overwrite, remove, or unchanged before touching disk.
// Why: destructive restores must show their full plan first; the plan is also what --force executes.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::target_entries::target_entries;
use super::types::{RestoreOptions, RestorePlan};
use crate::workspace::{selected, validate_relative};
use crate::Result;

pub fn plan_restore(opts: &RestoreOptions) -> Result<RestorePlan> {
    for path in opts.paths.unwrap_or_default() {
        validate_relative(path)?; // reject traversal before any comparison
    }
    let target = target_entries(opts.root, opts.commit_hash)?.entries; // comparable ids only; bytes are not needed to plan
    let current = super::current::scan(opts.root, opts.paths)?;
    let mut plan = RestorePlan {
        create: Vec::new(),
        overwrite: Vec::new(),
        remove: Vec::new(),
        unchanged: Vec::new(),
    };
    for (path, expected) in target.iter().filter(|(path, _)| selected(path, opts.paths)) {
        match current.get(path) {
            None => plan.create.push(path.clone()),
            Some(actual) if actual.tree_entry(path)? == *expected => {
                plan.unchanged.push(path.clone()) // identical kind, mode, target, and id
            }
            Some(_) => plan.overwrite.push(path.clone()),
        }
    }
    if opts.paths.is_none() {
        plan.remove.extend(
            current
                .keys()
                .filter(|path| !target.contains_key(*path))
                .cloned(), // whole-root restores also remove files the target does not have
        );
    }
    super::order::sort(&mut plan);
    Ok(plan)
}
