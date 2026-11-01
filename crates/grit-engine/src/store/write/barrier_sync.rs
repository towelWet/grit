// barrier_sync.rs — push one object to disk in order, without flushing whole drive cache.
// Why: Rust sync_all on macOS is F_FULLFSYNC; once per object stalled snapshots for minutes on
//      external drives. Barrier keeps objects ordered before commit, and commit still full-flushes.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use std::fs::File;
use std::io;

pub fn barrier_sync(file: &File) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::os::fd::AsRawFd;
        if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_BARRIERFSYNC) } == 0 {
            return Ok(());
        }
    }
    file.sync_all() // other OS, or filesystem without barrier support (exFAT): safe slow path
}
