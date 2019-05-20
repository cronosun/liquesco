use liquesco_common::error::LqError;
use crate::core::Context;
use crate::core::Type;
use crate::doc_type::DocType;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use liquesco_core::serialization::binary::Binary;
use liquesco_core::serialization::core::DeSerializer;
use liquesco_core::serialization::uuid::Uuid;
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct TUuid;

impl Type for TUuid {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // it's just a normal binary
        Uuid::de_serialize(context.reader())?;
        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        // compare like "normal" binaries
        let bin1 = Binary::de_serialize(r1)?;
        let bin2 = Binary::de_serialize(r2)?;
        Result::Ok(bin1.cmp(&bin2))
    }
}

impl BaseTypeSchemaBuilder for TUuid {
    fn build_schema<B>(_: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        DocType::from(TStruct::default())
    }
}
