// generated.rs — true for rebuildable output folders at any depth.
// Why: root-only "target/" rule missed nested dbv/.build and .build-tmp; each snapshot hashed them.
//      target is not here: nested folder named target can be real content.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use std::path::Path;

const NAMES: [&str; 5] = [".build", ".build-tmp", ".venv", "__pycache__", "node_modules"];

pub(super) fn generated(path: &Path) -> bool {
    path.components().any(|part| NAMES.iter().any(|name| part.as_os_str() == *name))
}
