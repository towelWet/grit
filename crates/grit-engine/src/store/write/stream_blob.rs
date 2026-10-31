// stream_blob.rs — copy a reader through BLAKE3 and zstd in fixed-size chunks.
// Why: chunked copies keep memory flat for any file size; the id is known once the last chunk is read.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::{blob_hasher, Hash};
use crate::objects::blob::RAW_MAGIC;
use crate::Result;
use std::fs::File;
use std::io::{Read, Write};

const CHUNK: usize = 64 * 1024; // streaming unit: working memory is one chunk, not one file

pub fn stream_blob<R: Read>(mut reader: R, file: &mut File) -> Result<(Hash, u64)> {
    let mut hasher = blob_hasher(); // already carries the "blob\0" domain tag
    let mut encoder = zstd::stream::Encoder::new(file, 3)?; // level 3 matches the JSON writer
    encoder.write_all(RAW_MAGIC)?; // marker first so readers can tell raw from JSON
    let mut buffer = vec![0u8; CHUNK];
    let mut size = 0u64;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break; // end of file
        }
        hasher.update(&buffer[..read]); // id covers exactly the file bytes
        encoder.write_all(&buffer[..read])?;
        size += read as u64;
    }
    encoder.finish()?; // flushes the final zstd frame; without it the object is truncated
    Ok((Hash(*hasher.finalize().as_bytes()), size))
}
