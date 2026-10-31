use super::Hash;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.hex())
        } else {
            self.0.serialize(serializer)
        }
    }
}

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

fn serializer_is_text<'de, D: Deserializer<'de>>(value: &D) -> bool {
    value.is_human_readable()
}
