#[derive(Debug)]
pub enum Command {
    Init,
    Status,
    Diff,
    Save,
    Log,
    Show,
    Undo,
    Restore,
    Checkout,
    Branch { sub: BranchSub },
    Tag { sub: TagSub },
    Verify,
    Gc,
    Doctor,
}

#[derive(Debug)]
pub enum BranchSub {
    List,
    Create { name: String },
}
