// safety_snapshot.rs — save the current state just before a destructive restore.
// Why: a restore that overwrites or removes files must leave a recoverable snapshot of what it displaced.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::types::RestoreOptions;
use crate::Result;

pub(super) fn safety_snapshot(opts: &RestoreOptions) -> Result<()> {
    let config = crate::config::GritConfig::load(opts.root)?;
    crate::snapshot::create_snapshot(&crate::snapshot::SnapshotOptions {
        root: opts.root,
        author: opts.author,
        message: "safety pre-restore", // recognizable in `grit log` if the user needs it
        paths: None,                   // always whole-root: a partial restore still protects everything
        ignore: &crate::ignore::IgnoreRules::new(&config.save.exclude),
        metadata: &std::collections::BTreeMap::new(),
    })?;
    Ok(())
}
