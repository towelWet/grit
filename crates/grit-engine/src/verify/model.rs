#[derive(Default)]
pub struct VerifyResult {
    pub total_objects: u64,
    pub valid: u64,
    pub corrupt: u64,
    pub errors: Vec<String>,
    pub reachable: u64,
    pub unreachable: u64,
}
