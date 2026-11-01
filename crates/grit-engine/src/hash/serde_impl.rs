use super::Hash;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
mod impl_serialize;
mod impl_deserialize;
mod serializer_is_text;
use serializer_is_text::serializer_is_text;
