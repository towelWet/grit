use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub kind: EntryKind,
    pub hash: Hash,
    pub mode: u32,
    #[serde(default)]
    pub target: Option<Vec<u8>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub entries: BTreeMap<String, TreeEntry>,
}
impl Tree {
    pub fn new(e: BTreeMap<String, TreeEntry>) -> Self {
        Self { entries: e }
    }
    pub fn empty() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
    pub fn entry(&self, name: &str) -> Option<&TreeEntry> {
        self.entries.get(name)
    }
}
