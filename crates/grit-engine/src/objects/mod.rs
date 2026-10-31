// mod.rs — object kinds and the re-exports every store/snapshot caller uses.
// Why: one import path (crate::objects) keeps blob, tree, and commit shapes discoverable together.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
pub mod blob;
pub mod commit;
pub mod empty_dir_hash;
pub mod payload_hash;
pub mod tree;
pub use blob::Blob;
pub use commit::Commit;
use serde::{Deserialize, Serialize};
pub use tree::{Tree, TreeEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
}

impl ObjectKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectKind::Blob => "blob",
            ObjectKind::Tree => "tree",
            ObjectKind::Commit => "commit",
        }
    }
}
