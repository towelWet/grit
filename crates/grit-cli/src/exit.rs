#[derive(Debug, Clone, Copy)]
pub enum ExitCode {
    Success,
    DataErr,
}
impl ExitCode {
    pub fn code(&self) -> i32 {
        match self {
            ExitCode::Success => 0,
            ExitCode::DataErr => 65,
        }
    }
}
