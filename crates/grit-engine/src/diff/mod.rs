// mod.rs — diff module: commit-to-commit and working-tree diffs, plus their result types.
// Why: callers get one import surface; each diff source lives in its own file.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod compare;
mod content;
mod diff_commits;
mod model;
mod text;
mod working;
mod working_content;

pub use diff_commits::diff_commits;
pub use model::{DiffEntry, DiffResult};
pub use working::diff_working_tree;
