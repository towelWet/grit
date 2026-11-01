// serializer_is_text.rs — one item split out of serde_impl.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(super) fn serializer_is_text<'de, D: Deserializer<'de>>(value: &D) -> bool {
    value.is_human_readable()
}
