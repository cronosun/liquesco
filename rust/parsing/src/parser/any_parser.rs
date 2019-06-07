use crate::parser::ascii::PAscii;
use crate::parser::binary::PBinary;
use crate::parser::boolean::PBool;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::enumeration::PEnum;
use crate::parser::float::PFloat32;
use crate::parser::float::PFloat64;
use crate::parser::key_ref::PKeyRef;
use crate::parser::map::PMap;
use crate::parser::option::POption;
use crate::parser::range::PRange;
use crate::parser::root_map::PRootMap;
use crate::parser::seq::PSeq;
use crate::parser::sint::PSInt;
use crate::parser::structure::PStruct;
use crate::parser::uint::PUInt;
use crate::parser::unicode::PUnicode;
use crate::parser::uuid::PUuid;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;

pub(crate) fn parse_any<'c, C>(
    context: &mut C,
    any_type: &AnyType,
    text_value: &TextValue,
    writer: &mut C::TWriter,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    match any_type {
        AnyType::Option(value) => POption::parse(context, writer, text_value, value),
        AnyType::UInt(value) => PUInt::parse(context, writer, text_value, value),
        AnyType::SInt(value) => PSInt::parse(context, writer, text_value, value),
        AnyType::Struct(value) => PStruct::parse(context, writer, text_value, value),
        AnyType::Seq(value) => PSeq::parse(context, writer, text_value, value),
        AnyType::Binary(value) => PBinary::parse(context, writer, text_value, value),
        AnyType::Ascii(value) => PAscii::parse(context, writer, text_value, value),
        AnyType::Enum(value) => PEnum::parse(context, writer, text_value, value),
        AnyType::Bool(value) => PBool::parse(context, writer, text_value, value),
        AnyType::Unicode(value) => PUnicode::parse(context, writer, text_value, value),
        AnyType::Float32(value) => PFloat32::parse(context, writer, text_value, value),
        AnyType::Float64(value) => PFloat64::parse(context, writer, text_value, value),
        AnyType::Uuid(value) => PUuid::parse(context, writer, text_value, value),
        AnyType::Range(value) => PRange::parse(context, writer, text_value, value),
        AnyType::Map(value) => PMap::parse(context, writer, text_value, value),
        AnyType::RootMap(value) => PRootMap::parse(context, writer, text_value, value),
        AnyType::KeyRef(value) => PKeyRef::parse(context, writer, text_value, value),
    }
}
