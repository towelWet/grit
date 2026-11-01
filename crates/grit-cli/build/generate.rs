use std::fs;
use std::path::{Path, PathBuf};
#[path = "generate/includes.rs"]   // crate root: Cargo looks beside this file, not in generate/
mod includes;
pub use includes::includes;
#[path = "generate/methods.rs"]   // crate root: Cargo looks beside this file, not in generate/
mod methods;
pub use methods::methods;
#[path = "generate/selected.rs"]   // crate root: Cargo looks beside this file, not in generate/
mod selected;
use selected::selected;
#[path = "generate/include_line.rs"]   // crate root: Cargo looks beside this file, not in generate/
mod include_line;
use include_line::include_line;
#[path = "generate/write.rs"]   // crate root: Cargo looks beside this file, not in generate/
mod write;
use write::write;
