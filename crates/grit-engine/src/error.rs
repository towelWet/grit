use thiserror::Error;
mod conversions;
#[derive(Error, Debug)]
pub enum GritError {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML ser: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("TOML de: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Zstd: {0}")]
    Zstd(String),
    #[error("Not a repo ({0})")]
    NotARepository(String),
    #[error("Already init: {0}")]
    AlreadyInitialized(String),
    #[error("Locked pid {0}")]
    Locked(u32),
    #[error("Obj not found: {0}")]
    ObjectNotFound(String),
    #[error("Obj corrupt {0}: exp {1} got {2}")]
    ObjectCorrupt(String, String, String),
    #[error("Invalid obj: {0}")]
    InvalidObject(String),
    #[error("Ref not found: {0}")]
    RefNotFound(String),
    #[error("Tag exists: {0}")]
    TagExists(String),
    #[error("Overwrite protected: {0}")]
    OverwriteProtected(String),
    #[error("Save not found: {0}")]
    SaveNotFound(String),
    #[error("Tx conflict: {0}")]
    TransactionConflict(String),
    #[error("Verify: {0}")]
    Verification(String),
    #[error("GC: {0}")]
    Gc(String),
    #[error("Internal: {0}")]
    Internal(String),
    #[error("Path traversal: {0}")]
    PathTraversal(String),
    #[error("Path crosses symlink: {0}")]
    SymlinkSelection(String), // --path routed through a symlink the walker won't follow
}
