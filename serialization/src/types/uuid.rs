use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::types::binary::Binary;
use liquesco_common::error::LqError;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::convert::TryFrom;

/// 16 byte Universally Unique Identifier (UUID) according to RFC 4122. Does not specify which
/// variant is allowed. Does not validate: Any 16 byte binary is allowed.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Uuid([u8; 16]);

impl<'a> DeSerializer<'a> for Uuid {
    type Item = Self;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        // it's just a normal binary
        let binary = Binary::de_serialize(reader)?;
        let mut uuid_bytes: [u8; 16] = [0; 16];
        let src_len = binary.len();
        if src_len != 16 {
            return LqError::err_new(format!(
                "Invalid length of UUID (need to be 16 bytes; have {:?} bytes)",
                src_len
            ));
        }
        uuid_bytes.clone_from_slice(binary);
        Result::Ok(Uuid(uuid_bytes))
    }
}

impl TryFrom<&[u8]> for Uuid {
    type Error = LqError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 16 {
            let mut uuid_bytes: [u8; 16] = [0; 16];
            uuid_bytes.clone_from_slice(value);
            Ok(Uuid(uuid_bytes))
        } else {
            Err(LqError::new(format!(
                "Given binary for uuid has invalid length (need 16 bytes), \
                 have {} bytes",
                value.len()
            )))
        }
    }
}

impl Serializer for Uuid {
    type Item = Self;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        Binary::serialize(writer, &item.0)
    }
}

impl Uuid {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn as_hex_string(&self) -> String {
        let mut result = String::with_capacity(self.0.len() * 2);
        for byte in &self.0 {
            result = result + &format!("{:x}", byte);
        }
        result
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl From<&[u8; 16]> for Uuid {
    fn from(value: &[u8; 16]) -> Self {
        Self(value.clone())
    }
}

/// Need custom serde (compact serialization required).
impl serde::Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

/// Need custom serde (compact serialization required).
impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(UuidVisitor)
    }
}

struct UuidVisitor;

impl<'de> serde::de::Visitor<'de> for UuidVisitor {
    type Value = Uuid;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("Binary data (used for uuid)")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Uuid::try_from(v).map_err(|lq_err| E::custom(format!("{:?}", lq_err)))
    }
}
