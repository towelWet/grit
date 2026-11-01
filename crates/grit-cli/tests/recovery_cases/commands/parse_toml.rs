// parse_toml.rs — one item split out of commands.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub(super) fn parse_toml(value: &str) {
    toml::from_str::<toml::Value>(value).unwrap();
}
