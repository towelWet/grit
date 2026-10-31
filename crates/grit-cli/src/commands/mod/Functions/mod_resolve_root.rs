fn resolve_root(args: &GritArgs) -> PathBuf {
    args.repo
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}
