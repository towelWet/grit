use crate::objects::TreeEntry;
use std::collections::{BTreeMap, BTreeSet};

pub type Changes = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);
mod classify;
pub use classify::classify;
mod same_content;
pub use same_content::same_content;
