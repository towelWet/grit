    fn defaults() -> Self {
        Self {
            command: Command::Status,
            repo: None,
            format: "text".into(),
            message: None,
            author: "user".into(),
            paths: Vec::new(),
            force: false,
            id: None,
            snapshot_a: None,
            snapshot_b: None,
            limit: 20,
            metadata: Vec::new(),
            no_safety_save: false,
            branch_name: None,
        }
    }
