// commit.rs — one saved state: root tree pointer, parents, provenance, and the format it was written in.
// Why: `format` tells readers which blob-id scheme and tree layout apply, so old commits keep working.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod new;

// Commits written before `format` existed deserialize to 0: JSON blobs, flat tree, path-hash ids.
pub const FORMAT_LEGACY: u32 = 0;
// Raw blob payloads, blob ids from Hash::of_blob, still a flat tree keyed by full path.
pub const FORMAT_RAW_BLOBS: u32 = 1;
// Raw blobs plus one tree object per directory; a directory entry's hash is its subtree's id.
pub const FORMAT_NESTED: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub root_tree: Hash,
    pub parents: Vec<Hash>,
    pub author: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    #[serde(default)] // absent in pre-branch commits
    pub branch: Option<String>,
    #[serde(default)] // absent in pre-metadata commits
    pub metadata: std::collections::BTreeMap<String, String>,
    #[serde(default)] // absent in every commit written before the raw-blob format
    pub format: u32,
    #[serde(default)] // only paths that differ from the parent; empty for legacy commits
    pub changed_paths: Vec<String>,
}
