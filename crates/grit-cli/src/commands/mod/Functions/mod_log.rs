fn log(root: &Path, limit: usize, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let h = match refs::get_head_hash(&r.root) {
        Ok(h) => h,
        Err(_) => {
            println!("(no commits)");
            return Ok(ExitCode::Success);
        }
    };
    let mut hashes = vec![h];
    let mut cur = h;
    while let Ok(commit) = store::read::read_commit(&r.root, &cur) {
        if let Some(parent) = commit.parents.first() {
            hashes.push(*parent);
            cur = *parent;
        } else {
            break;
        }
        if hashes.len() >= limit {
            break;
        }
    }
    if fmt == "toml" {
        let entries: Vec<_> = hashes.iter().filter_map(|hash| {
            store::read::read_commit(&r.root, hash).ok().map(|commit| {
                serde_json::json!({"hash":hash.hex(),"message":commit.message,"author":commit.author,"timestamp":commit.timestamp.to_rfc3339()})
            })
        }).collect();
        println!("{}", report::format_toml(&serde_json::json!({"entries": entries})))
    } else {
        for h in &hashes {
            if let Ok(c) = store::read::read_commit(&r.root, h) {
                println!("{}  {}  {}", h.short(), c.author, c.message);
            }
        }
    }
    Ok(ExitCode::Success)
}
