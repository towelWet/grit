use crate::{GritError, Hash, Result};
use std::path::Path;
mod resolve_ref;
pub use resolve_ref::resolve_ref;
mod resolve_revision;
pub use resolve_revision::resolve_revision;
