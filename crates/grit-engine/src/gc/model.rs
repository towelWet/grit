#[derive(Default)]
pub struct GcResult {
    pub removed: u64,
    pub bytes_freed: u64,
    pub kept: u64,
}
