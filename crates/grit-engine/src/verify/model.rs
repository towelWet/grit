#[derive(Default)]
pub struct VerifyResult {
    pub total_objects: u64,
    pub valid: u64,
    pub cached: u64, // valid without rereading this run
    pub corrupt: u64,
    pub errors: Vec<String>,
    pub reachable: u64,
    pub unreachable: u64,
}
