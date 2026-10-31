    fn command_from(&mut self, words: &[String]) -> Result<Command, String> {
        let Some(command) = words.first().map(String::as_str) else {
            return Ok(Command::Status);
        };
        let rest = &words[1..];
        match command {
            "init" => Ok(Command::Init),
            "status" => Ok(Command::Status),
            "diff" => {
                self.snapshot_a = rest.first().cloned();
                self.snapshot_b = rest.get(1).cloned();
                Ok(Command::Diff)
            }
            "save" | "snapshot" => Ok(Command::Save),
            "log" => Ok(Command::Log),
            "show" => {
                self.id = self.id.take().or_else(|| rest.first().cloned());
                Ok(Command::Show)
            }
            "undo" => Ok(Command::Undo),
            "restore" => {
                self.id = self.id.take().or_else(|| rest.first().cloned());
                Ok(Command::Restore)
            }
            "checkout" => {
                self.branch_name = rest.first().cloned();
                Ok(Command::Checkout)
            }
            "branch" if rest.first().map(String::as_str) == Some("create") => Ok(Command::Branch {
                sub: BranchSub::Create {
                    name: required(rest.get(1), "branch name")?,
                },
            }),
            "branch" => Ok(Command::Branch {
                sub: BranchSub::List,
            }),
            "tag" if rest.first().map(String::as_str) == Some("create") => Ok(Command::Tag {
                sub: TagSub::Create {
                    name: required(rest.get(1), "tag name")?,
                    hash: rest.get(2).cloned().unwrap_or_default(),
                },
            }),
            "tag" => Ok(Command::Tag { sub: TagSub::List }),
            "verify" => Ok(Command::Verify),
            "gc" => Ok(Command::Gc),
            "doctor" => Ok(Command::Doctor),
            other => Err(format!("unknown command: {other}")),
        }
    }
