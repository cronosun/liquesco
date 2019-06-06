use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::BaseTypeSchemaBuilder;
use crate::schema_builder::SchemaBuilder;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::uint::UInt32;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::context::CmpContext;

/// References a key in the next outer map.
///
/// Technical detail: It's just a number. That number is the index in the outer map. So it's
/// always >=0 and < number of items in the map. It can only be used when there's an outer
/// map in the schema. When there's no outer map, schema is valid but data validation will
/// always fail.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TKeyRef<'a> {
    meta: Meta<'a>,
}

impl<'a> Default for TKeyRef<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
        }
    }
}

impl Type for TKeyRef<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let ref_int = UInt32::de_serialize(context.reader())?;
        if let Some(len) = context.key_ref_info().map_len() {
            if ref_int >= len {
                LqError::err_new(format!(
                    "You're referencing key at index {} in a map but \
                     the map only has {} keys.",
                    ref_int, len
                ))
            } else {
                Ok(())
            }
        } else {
            LqError::err_new(format!(
                "You're trying to reference key {} in a map but \
                 there's no map that's currently being processed. Key references can only \
                 be within a map.",
                ref_int
            ))
        }
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let int1 = UInt32::de_serialize(r1)?;
        let int2 = UInt32::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TKeyRef<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TKeyRef<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TKeyRef<'_> {
    fn build_schema<B>(_: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        TStruct::default().with_doc(
            "Key references can reference keys from outer types that supports references \
             (provide anchors that can be referenced): Maps and RootMaps.",
        )
    }
}
