// create.rs — the whole snapshot: lock, prepare, classify, write the tree and commit, then refresh the cache.
// Why: the lock serializes writers, and the cache is saved only after the commit so it never outruns the history.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::classify::classify;
use super::collect_changed::collect_changed;
use super::dirty_dirs::dirty_dirs;
use super::no_change_result::no_change_result;
use super::persist::{persist_commit, persist_tree};
use super::prepare::prepare;
use super::store_kept::store_kept;
use super::types::{ObjectStats, SnapshotOptions, SnapshotResult};
use crate::store::lock::Lock;
use crate::Result;

pub fn create_snapshot(opts: &SnapshotOptions) -> Result<SnapshotResult> {
    let _lock = Lock::acquire(opts.root)?; // one writer at a time; released when this function returns
    let prepared = prepare(opts)?;
    let (added, modified, deleted, renamed) = classify(&prepared.old, &prepared.new);
    let changed_files = collect_changed(&added, &modified, &deleted, &renamed); // one list for commit and journal
    if changed_files.is_empty() {
        prepared.index.save(opts.root)?; // the cache still refreshes; no history entry is needed
        return Ok(no_change_result(ObjectStats::default())); // a clean tree writes no objects and records nothing
    }
    let mut stats = store_kept(opts.root, &prepared.kept)?; // only now do the changed objects get written
    let dirty = dirty_dirs(&prepared.old, &prepared.new); // directories whose trees must be rewritten
    let tree_hash = persist_tree(opts.root, &prepared.new, &dirty, prepared.old_root, &mut stats)?;
    let (commit_hash, branch) = persist_commit(
        opts,
        tree_hash,
        prepared.parent,
        &changed_files,
        &mut stats,
    )?;
    prepared.index.save(opts.root)?; // only after the commit, so cached ids always name stored objects
    crate::journal::append(opts.root, "snapshot", &commit_hash, &branch, &changed_files)?;
    Ok(SnapshotResult {
        commit_hash: Some(commit_hash),
        tree_hash: Some(tree_hash),
        object_count: stats.count,
        bytes_added: stats.added,
        bytes_reused: stats.reused,
        changed_files,
        modified,
        added,
        deleted,
        renamed,
    })
}
