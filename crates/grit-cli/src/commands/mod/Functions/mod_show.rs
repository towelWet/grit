fn show(root: &Path, id: &Option<String>, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let hash = match id {
        Some(s) => refs::resolve_revision(&r.root, s)?,
        None => refs::get_head_hash(&r.root)?,
    };
    let c = store::read::read_commit(&r.root, &hash)?;
    if fmt == "toml" {
        println!(
            "{}",
            report::format_toml(
                &serde_json::json!({"hash":hash.hex(),"tree":c.root_tree.hex(),"parents":c.parents.iter().map(|p|p.hex()).collect::<Vec<_>>(),"author":c.author,"message":c.message,"timestamp":c.timestamp.to_rfc3339()})
            )
        )
    } else {
        println!(
            "Save: {}\nTree: {}\nAuthor: {}\nDate: {}\nMessage: {}",
            hash.hex(),
            c.root_tree.hex(),
            c.author,
            c.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            c.message
        );
    }
    Ok(ExitCode::Success)
}
