mod ancestor; // `<rev>~N` resolution
mod branch;
mod head;
mod name;
mod prefix;
mod resolve;
mod tag;

pub use branch::{get_branch_ref, list_branches, set_branch_ref};
pub use head::{get_current_branch, get_head, get_head_hash, set_head};
pub use resolve::{resolve_ref, resolve_revision};
pub use tag::{get_tag, list_tags, set_tag};
