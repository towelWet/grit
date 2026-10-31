pub fn delta(
    path: &str,
    old: Option<&[u8]>,
    new: Option<&[u8]>,
) -> (Option<u64>, Option<u64>, Option<String>) {
    let (old, new) = (old.unwrap_or_default(), new.unwrap_or_default());
    if old.contains(&0) || new.contains(&0) {
        return (None, None, None);
    }
    let (Ok(old), Ok(new)) = (std::str::from_utf8(old), std::str::from_utf8(new)) else {
        return (None, None, None);
    };
    let (old, new): (Vec<_>, Vec<_>) = (old.lines().collect(), new.lines().collect());
    let mut prefix = 0;
    while prefix < old.len().min(new.len()) && old[prefix] == new[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < old.len().saturating_sub(prefix)
        && suffix < new.len().saturating_sub(prefix)
        && old[old.len() - 1 - suffix] == new[new.len() - 1 - suffix]
    {
        suffix += 1;
    }
    let removed = &old[prefix..old.len().saturating_sub(suffix)];
    let added = &new[prefix..new.len().saturating_sub(suffix)];
    let mut patch = format!("--- a/{path}\n+++ b/{path}\n");
    for line in removed {
        patch.push_str(&format!("-{line}\n"));
    }
    for line in added {
        patch.push_str(&format!("+{line}\n"));
    }
    (
        Some(added.len() as u64),
        Some(removed.len() as u64),
        Some(patch),
    )
}
