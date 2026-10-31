// persist_commit.rs — write the commit for a snapshot and advance its branch.
// Why: the commit is the only point where the new state becomes visible in history, so it comes last.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::Commit;
use crate::snapshot::types::{ObjectStats, SnapshotOptions};
use crate::store::write::write_json;
use crate::Result;

pub fn persist_commit(
    opts: &SnapshotOptions,
    tree: Hash,
    parent: Option<Hash>,
    changed: &[String], // only the paths that differ from the parent are recorded
    stats: &mut ObjectStats,
) -> Result<(Hash, String)> {
    let branch = crate::refs::get_current_branch(opts.root).unwrap_or_else(|_| "main".into());
    let mut commit = Commit::new(
        tree,
        parent.into_iter().collect(),
        opts.author,
        opts.message,
    );
    commit.branch = Some(branch.clone());
    commit.metadata = opts.metadata.clone();
    commit.changed_paths = changed.to_vec(); // the root pointer already names the full state
    let hash = write_json(opts.root, &commit)?;
    stats.count += 1;
    crate::refs::set_branch_ref(opts.root, &branch, &hash)?; // the branch moves only after the commit is stored
    Ok((hash, branch))
}
