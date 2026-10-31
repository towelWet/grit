use crate::args::{BranchSub, Command, GritArgs, TagSub};
use crate::exit::ExitCode;
use crate::output;
use grit_engine::*;
use std::path::{Path, PathBuf};

type CmdResult = std::result::Result<ExitCode, Box<dyn std::error::Error>>;
