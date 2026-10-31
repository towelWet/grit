#[derive(Debug)]
pub enum TagSub {
    List,
    Create { name: String, hash: String },
}

#[derive(Debug)]
pub struct GritArgs {
    pub command: Command,
    pub repo: Option<PathBuf>,
    pub format: String,
    pub message: Option<String>,
    pub author: String,
    pub paths: Vec<String>,
    pub force: bool,
    pub id: Option<String>,
    pub snapshot_a: Option<String>,
    pub snapshot_b: Option<String>,
    pub limit: usize,
    pub metadata: Vec<String>,
    pub no_safety_save: bool,
    pub branch_name: Option<String>,
}
