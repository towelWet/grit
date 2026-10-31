fn next(words: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    words
        .next()
        .ok_or_else(|| format!("{option} requires a value"))
}
