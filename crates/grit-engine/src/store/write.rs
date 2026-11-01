// write.rs — object writers: JSON objects, streamed raw blobs, and the atomic helpers they share.
// Why: objects are content-addressed, so a writer only needs to create a file that does not exist yet.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod atomic;
mod barrier_sync;
mod finalize_object;
mod is_unsupported; // helper split out of persist_object so each file holds one function
mod persist_object;
mod stream_blob;
mod temp_object;
mod write_blob;
mod write_json;
mod write_object;

pub use atomic::atomic_write;
pub use finalize_object::finalize_object;
pub use stream_blob::stream_blob;
pub use temp_object::temp_object;
pub use write_blob::write_blob;
pub use write_json::write_json;
pub use write_object::write_object;
