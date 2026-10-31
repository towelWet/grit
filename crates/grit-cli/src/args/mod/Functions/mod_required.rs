fn required(value: Option<&String>, label: &str) -> Result<String, String> {
    value.cloned().ok_or_else(|| format!("missing {label}"))
}
