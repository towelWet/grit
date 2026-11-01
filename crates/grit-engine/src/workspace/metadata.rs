use crate::Result;
use std::fs;
use std::path::Path;
#[cfg(unix)]
mod file_mode;
#[cfg(unix)]
pub use file_mode::file_mode;
#[cfg(not(unix))]
mod file_mode_;
#[cfg(not(unix))]
pub use file_mode_::file_mode;
#[cfg(unix)]
mod symlink_target;
#[cfg(unix)]
pub use symlink_target::symlink_target;
#[cfg(not(unix))]
mod symlink_target_;
#[cfg(not(unix))]
pub use symlink_target_::symlink_target;
