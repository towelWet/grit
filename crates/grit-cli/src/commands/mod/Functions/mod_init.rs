fn init(root: &Path) -> CmdResult {
    let r = Repository::init(root)?;
    println!("Initialized empty grit repo in {}", r.grit_dir().display());
    Ok(ExitCode::Success)
}
