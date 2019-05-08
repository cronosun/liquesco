use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::tbinary::TBinary;
use crate::common::error::LqError;
use crate::serialization::core::Serializer;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Uuid([u8; 16]);

impl<'a> DeSerializer<'a> for Uuid {
    type Item = Self;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        // it's just a normal binary
        let binary = TBinary::de_serialize(reader)?;
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

impl Serializer for Uuid {
    type Item = Self;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        TBinary::serialize(writer, &item.0)        
    }
}

impl Uuid {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn new_v4() -> Self {
        let new_v4 = uuid::Uuid::new_v4();
        Self(new_v4.as_bytes().clone())
    }
}
