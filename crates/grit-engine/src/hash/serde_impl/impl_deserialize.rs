// impl_deserialize.rs — one item split out of serde_impl.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if serializer_is_text(&deserializer) {
            let value = <&str>::deserialize(deserializer)?;
            let mut output = [0u8; 32];
            hex::decode_to_slice(value, &mut output).map_err(D::Error::custom)?;
            Ok(Hash(output))
        } else {
            Ok(Hash(<[u8; 32]>::deserialize(deserializer)?))
        }
    }
}
