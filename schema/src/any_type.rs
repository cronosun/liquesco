use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::WithMetaSchemaBuilder;
use crate::metadata::{MetadataSetter, WithMetadata};
use crate::schema_builder::BaseTypeSchemaBuilder;
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use crate::types::ascii::TAscii;
use crate::types::binary::TBinary;
use crate::types::boolean::TBool;
use crate::types::decimal::TDecimal;
use crate::types::enumeration::TEnum;
use crate::types::enumeration::Variant;
use crate::types::float::TFloat32;
use crate::types::float::TFloat64;
use crate::types::key_ref::TKeyRef;
use crate::types::map::TMap;
use crate::types::option::TOption;
use crate::types::range::TRange;
use crate::types::root_map::TRootMap;
use crate::types::seq::TSeq;
use crate::types::sint::TSInt;
use crate::types::structure::TStruct;
use crate::types::uint::TUInt;
use crate::types::unicode::TUnicode;
use crate::types::uuid::TUuid;
use from_variants::FromVariants;
use liquesco_common::error::LqError;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// This is an enumeration of all `Type`s that are known to the system.
///
/// Note: Sorted according to serialization major type.
#[derive(Clone, FromVariants, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnyType<'a> {
    Bool(TBool<'a>),
    Option(TOption<'a>),
    Seq(TSeq<'a>),
    Binary(TBinary<'a>),
    Unicode(TUnicode<'a>),
    UInt(TUInt<'a>),
    SInt(TSInt<'a>),
    Float32(TFloat32<'a>),
    Float64(TFloat64<'a>),
    Enum(TEnum<'a>),

    Struct(TStruct<'a>),
    Map(TMap<'a>),
    RootMap(TRootMap<'a>),
    KeyRef(TKeyRef<'a>),
    Ascii(TAscii<'a>),
    Uuid(TUuid<'a>),
    Range(TRange<'a>),
    Decimal(TDecimal<'a>),
}

impl<'a> WithMetadata for AnyType<'a> {
    fn meta(&self) -> &Meta {
        match self {
            AnyType::Struct(value) => value.meta(),
            AnyType::Map(value) => value.meta(),
            AnyType::RootMap(value) => value.meta(),
            AnyType::KeyRef(value) => value.meta(),
            AnyType::UInt(value) => value.meta(),
            AnyType::SInt(value) => value.meta(),
            AnyType::Ascii(value) => value.meta(),
            AnyType::Bool(value) => value.meta(),
            AnyType::Enum(value) => value.meta(),
            AnyType::Seq(value) => value.meta(),
            AnyType::Binary(value) => value.meta(),
            AnyType::Float32(value) => value.meta(),
            AnyType::Float64(value) => value.meta(),
            AnyType::Option(value) => value.meta(),
            AnyType::Unicode(value) => value.meta(),
            AnyType::Uuid(value) => value.meta(),
            AnyType::Range(value) => value.meta(),
            AnyType::Decimal(value) => value.meta(),
        }
    }
}

impl<'a> Type for AnyType<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        // is there no macro for this?
        match self {
            AnyType::Struct(value) => value.validate(context),
            AnyType::Map(value) => value.validate(context),
            AnyType::RootMap(value) => value.validate(context),
            AnyType::KeyRef(value) => value.validate(context),
            AnyType::UInt(value) => value.validate(context),
            AnyType::SInt(value) => value.validate(context),
            AnyType::Ascii(value) => value.validate(context),
            AnyType::Bool(value) => value.validate(context),
            AnyType::Enum(value) => value.validate(context),
            AnyType::Seq(value) => value.validate(context),
            AnyType::Binary(value) => value.validate(context),
            AnyType::Float32(value) => value.validate(context),
            AnyType::Float64(value) => value.validate(context),
            AnyType::Option(value) => value.validate(context),
            AnyType::Unicode(value) => value.validate(context),
            AnyType::Uuid(value) => value.validate(context),
            AnyType::Range(value) => value.validate(context),
            AnyType::Decimal(value) => value.validate(context),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        // is there no macro for this?
        match self {
            AnyType::Struct(value) => value.compare(context, r1, r2),
            AnyType::Map(value) => value.compare(context, r1, r2),
            AnyType::RootMap(value) => value.compare(context, r1, r2),
            AnyType::KeyRef(value) => value.compare(context, r1, r2),
            AnyType::UInt(value) => value.compare(context, r1, r2),
            AnyType::SInt(value) => value.compare(context, r1, r2),
            AnyType::Ascii(value) => value.compare(context, r1, r2),
            AnyType::Bool(value) => value.compare(context, r1, r2),
            AnyType::Enum(value) => value.compare(context, r1, r2),
            AnyType::Seq(value) => value.compare(context, r1, r2),
            AnyType::Binary(value) => value.compare(context, r1, r2),
            AnyType::Float32(value) => value.compare(context, r1, r2),
            AnyType::Float64(value) => value.compare(context, r1, r2),
            AnyType::Option(value) => value.compare(context, r1, r2),
            AnyType::Unicode(value) => value.compare(context, r1, r2),
            AnyType::Uuid(value) => value.compare(context, r1, r2),
            AnyType::Range(value) => value.compare(context, r1, r2),
            AnyType::Decimal(value) => value.compare(context, r1, r2),
        }
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        match self {
            AnyType::Struct(value) => value.reference(index),
            AnyType::Map(value) => value.reference(index),
            AnyType::RootMap(value) => value.reference(index),
            AnyType::KeyRef(value) => value.reference(index),
            AnyType::UInt(value) => value.reference(index),
            AnyType::SInt(value) => value.reference(index),
            AnyType::Ascii(value) => value.reference(index),
            AnyType::Bool(value) => value.reference(index),
            AnyType::Enum(value) => value.reference(index),
            AnyType::Seq(value) => value.reference(index),
            AnyType::Binary(value) => value.reference(index),
            AnyType::Float32(value) => value.reference(index),
            AnyType::Float64(value) => value.reference(index),
            AnyType::Option(value) => value.reference(index),
            AnyType::Unicode(value) => value.reference(index),
            AnyType::Uuid(value) => value.reference(index),
            AnyType::Range(value) => value.reference(index),
            AnyType::Decimal(value) => value.reference(index),
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        match self {
            AnyType::Struct(value) => value.set_reference(index, type_ref),
            AnyType::Map(value) => value.set_reference(index, type_ref),
            AnyType::RootMap(value) => value.set_reference(index, type_ref),
            AnyType::KeyRef(value) => value.set_reference(index, type_ref),
            AnyType::UInt(value) => value.set_reference(index, type_ref),
            AnyType::SInt(value) => value.set_reference(index, type_ref),
            AnyType::Ascii(value) => value.set_reference(index, type_ref),
            AnyType::Bool(value) => value.set_reference(index, type_ref),
            AnyType::Enum(value) => value.set_reference(index, type_ref),
            AnyType::Seq(value) => value.set_reference(index, type_ref),
            AnyType::Binary(value) => value.set_reference(index, type_ref),
            AnyType::Float32(value) => value.set_reference(index, type_ref),
            AnyType::Float64(value) => value.set_reference(index, type_ref),
            AnyType::Option(value) => value.set_reference(index, type_ref),
            AnyType::Unicode(value) => value.set_reference(index, type_ref),
            AnyType::Uuid(value) => value.set_reference(index, type_ref),
            AnyType::Range(value) => value.set_reference(index, type_ref),
            AnyType::Decimal(value) => value.set_reference(index, type_ref),
        }
    }
}

impl<'a> MetadataSetter<'a> for AnyType<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        match self {
            AnyType::Struct(value) => value.set_meta(meta),
            AnyType::Map(value) => value.set_meta(meta),
            AnyType::RootMap(value) => value.set_meta(meta),
            AnyType::KeyRef(value) => value.set_meta(meta),
            AnyType::UInt(value) => value.set_meta(meta),
            AnyType::SInt(value) => value.set_meta(meta),
            AnyType::Ascii(value) => value.set_meta(meta),
            AnyType::Bool(value) => value.set_meta(meta),
            AnyType::Enum(value) => value.set_meta(meta),
            AnyType::Seq(value) => value.set_meta(meta),
            AnyType::Binary(value) => value.set_meta(meta),
            AnyType::Float32(value) => value.set_meta(meta),
            AnyType::Float64(value) => value.set_meta(meta),
            AnyType::Option(value) => value.set_meta(meta),
            AnyType::Unicode(value) => value.set_meta(meta),
            AnyType::Uuid(value) => value.set_meta(meta),
            AnyType::Range(value) => value.set_meta(meta),
            AnyType::Decimal(value) => value.set_meta(meta),
        }
    }
}

impl BuildsOwnSchema for AnyType<'_> {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder<'static>,
    {
        let ref_bool = doc_type_ref::<TBool, B>("bool", builder);
        let ref_option = doc_type_ref::<TOption, B>("option", builder);
        let ref_seq = doc_type_ref::<TSeq, B>("seq", builder);
        let ref_binary = doc_type_ref::<TBinary, B>("binary", builder);
        let ref_unicode = doc_type_ref::<TUnicode, B>("unicode", builder);
        let ref_uint = doc_type_ref::<TUInt, B>("uint", builder);
        let ref_sint = doc_type_ref::<TSInt, B>("sint", builder);
        let ref_float_32 = doc_type_ref::<TFloat32, B>("float_32", builder);
        let ref_float_64 = doc_type_ref::<TFloat64, B>("float_64", builder);
        let ref_enum = doc_type_ref::<TEnum, B>("enum", builder);
        let ref_struct = doc_type_ref::<TStruct, B>("struct", builder);
        let ref_map = doc_type_ref::<TMap, B>("map", builder);
        let ref_root_map = doc_type_ref::<TRootMap, B>("root_map", builder);
        let ref_key_ref = doc_type_ref::<TKeyRef, B>("key_ref", builder);
        let ref_ascii = doc_type_ref::<TAscii, B>("ascii", builder);
        let ref_uuid = doc_type_ref::<TUuid, B>("uuid", builder);
        let ref_range = doc_type_ref::<TRange, B>("range", builder);
        let ref_decimal = doc_type_ref::<TDecimal, B>("decimal", builder);

        builder.add_unwrap(
            "any_type",
            TEnum::default()
                .add_variant(variant(ref_bool, "bool"))
                .add_variant(variant(ref_option, "option"))
                .add_variant(variant(ref_seq, "seq"))
                .add_variant(variant(ref_binary, "binary"))
                .add_variant(variant(ref_unicode, "unicode"))
                .add_variant(variant(ref_uint, "uint"))
                .add_variant(variant(ref_sint, "sint"))
                .add_variant(variant(ref_float_32, "f32"))
                .add_variant(variant(ref_float_64, "f64"))
                .add_variant(variant(ref_enum, "enum"))
                .add_variant(variant(ref_struct, "struct"))
                .add_variant(variant(ref_map, "map"))
                .add_variant(variant(ref_root_map, "root_map"))
                .add_variant(variant(ref_key_ref, "key_ref"))
                .add_variant(variant(ref_ascii, "ascii"))
                .add_variant(variant(ref_uuid, "uuid"))
                .add_variant(variant(ref_range, "range"))
                .add_variant(variant(ref_decimal, "decimal"))
                .with_doc(
                    "The any type is an enumeration of all possible types available \
                     in the type system.",
                ),
        )
    }
}

fn variant(reference: TypeRef, id: &'static str) -> Variant<'static> {
    Variant::new(Identifier::try_from(id).unwrap()).add_value(reference)
}

fn doc_type_ref<T, B>(id: &'static str, builder: &mut B) -> TypeRef
where
    T: BaseTypeSchemaBuilder + Type,
    B: SchemaBuilder<'static>,
{
    let schema = WithMetaSchemaBuilder::<T>::build_schema(builder);
    builder.add_unwrap(id, schema)
}
