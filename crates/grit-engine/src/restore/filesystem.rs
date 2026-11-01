use crate::{GritError, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
mod atomic_file;
pub use atomic_file::atomic_file;
#[cfg(unix)]
mod create_symlink;
#[cfg(unix)]
pub use create_symlink::create_symlink;
#[cfg(not(unix))]
mod create_symlink_;
#[cfg(not(unix))]
pub use create_symlink_::create_symlink;
#[cfg(unix)]
mod set_mode;
#[cfg(unix)]
pub use set_mode::set_mode;
#[cfg(not(unix))]
mod set_mode_;
#[cfg(not(unix))]
pub use set_mode_::set_mode;
