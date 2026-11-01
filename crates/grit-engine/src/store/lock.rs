use std::fs;
use std::path::PathBuf;
mod acquire;

pub struct Lock {
    path: PathBuf,
    acquired: bool,
}
mod release;
mod impl_drop;
