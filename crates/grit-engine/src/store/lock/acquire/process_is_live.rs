// process_is_live.rs — one item split out of acquire.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub(super) fn process_is_live(pid: u32) -> bool {
    unsafe { libc::kill(pid as i32, 0) == 0 }
}
