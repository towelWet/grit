// vanished.rs — true when an error means "file disappeared after the walk listed it".
// Why: lock/temp files (e.g. a LaunchAgent's tick.lock) are deleted between walk and
// read; that aborted a 7-minute baseline snapshot of ~/.claude/skills (2026-10-29).
// A path gone before we read it is simply not in this snapshot — same as deleted.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::GritError;

pub(crate) fn vanished(error: &GritError) -> bool {
    matches!(error, GritError::Io(io) if io.kind() == std::io::ErrorKind::NotFound) // only ENOENT; EPERM etc. still fail loudly
}
