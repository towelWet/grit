// context.rs — attach the offending path to workspace IO errors.
// Why: a bare "IO: Permission denied (os error 13)" hid which file macOS
// TCC blocked, so a home-rooted repo looked broken with no next step.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::GritError;
use std::path::Path;

pub(crate) fn with_path(error: GritError, path: &Path) -> GritError {
    match error {
        GritError::Io(io) => GritError::Io(std::io::Error::new(
            io.kind(),                                    // keep kind so callers can still match
            format!("{}: {io} (scope with --path)", path.display()), // name file + the fix
        )),
        other => other,                                   // non-IO errors already carry context
    }
}
