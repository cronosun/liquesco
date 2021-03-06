#![allow(clippy::derive_hash_xor_eq)]

use crate::core::ContentDescription;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_FLOAT;
use liquesco_common::error::LqError;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;
use std::hash::Hasher;

/// 32 bit or 64 bit float.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::F32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::F64(value)
    }
}

impl Float {
    fn try_into_f32(&self) -> Result<f32, LqError> {
        if let Float::F32(f32_value) = self {
            Result::Ok(*f32_value)
        } else {
            LqError::err_new(format!(
                "Given value is a float 64 - want a float 32; \
                 value {:?}",
                self
            ))
        }
    }

    fn try_into_f64(&self) -> Result<f64, LqError> {
        if let Float::F64(f64_value) = self {
            Result::Ok(*f64_value)
        } else {
            LqError::err_new(format!(
                "Given value is a float 32 - want a float 64; \
                 value {:?}",
                self
            ))
        }
    }
}

impl TryFrom<&Float> for f32 {
    type Error = LqError;
    fn try_from(value: &Float) -> Result<Self, Self::Error> {
        value.try_into_f32()
    }
}

impl TryFrom<&Float> for f64 {
    type Error = LqError;
    fn try_from(value: &Float) -> Result<Self, Self::Error> {
        value.try_into_f64()
    }
}

impl<'a> DeSerializer<'a> for Float {
    type Item = Float;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let float_type = begin_de_serialize(reader)?;
        match float_type {
            Type::F32 => Result::Ok(Float::F32(reader.read_f32()?)),
            Type::F64 => Result::Ok(Float::F64(reader.read_f64()?)),
        }
    }
}

impl Serializer for Float {
    type Item = Float;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Float::F32(value) => {
                begin_serialize(writer, Type::F32)?;
                writer.write_f32(*value)
            }
            Float::F64(value) => {
                begin_serialize(writer, Type::F64)?;
                writer.write_f64(*value)
            }
        }
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Float::F32(value) => {
                state.write_i8(4);
                state.write_u32(value.to_bits());
            }
            Float::F64(value) => {
                state.write_i8(8);
                state.write_u64(value.to_bits());
            }
        }
    }
}

pub struct Float32;

impl<'a> DeSerializer<'a> for Float32 {
    type Item = f32;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let float_type = begin_de_serialize(reader)?;
        if float_type == Type::F32 {
            reader.read_f32()
        } else {
            LqError::err_new("It's a 64 bit float and not a 32 bit float (expected).")
        }
    }
}

impl Serializer for Float32 {
    type Item = f32;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        begin_serialize(writer, Type::F32)?;
        writer.write_f32(*item)
    }
}

pub struct Float64;

impl<'a> DeSerializer<'a> for Float64 {
    type Item = f64;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let float_type = begin_de_serialize(reader)?;
        if float_type == Type::F64 {
            reader.read_f64()
        } else {
            LqError::err_new("It's a 32 bit float and not a 64 bit float (expected).")
        }
    }
}

impl Serializer for Float64 {
    type Item = f64;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        begin_serialize(writer, Type::F64)?;
        writer.write_f64(*item)
    }
}

#[inline]
fn begin_de_serialize<'a, R: LqReader<'a>>(reader: &mut R) -> Result<Type, LqError> {
    let type_header = reader.read_header_byte()?;
    let content_description = reader.read_content_description_given_header_byte(type_header)?;

    if type_header.major_type() != TYPE_FLOAT {
        return LqError::err_new("Given type is not a float type");
    }
    if content_description.number_of_embedded_items() != 0 {
        return LqError::err_new("Float types must not contain embedded values.");
    }

    match content_description.self_length() {
        4 => Result::Ok(Type::F32),
        8 => Result::Ok(Type::F64),
        n => LqError::err_new(format!(
            "Float has invalid number of bytes ({:?}); supported \
             floats are float32 (4 bytes) and float64 (8 bytes).",
            n
        )),
    }
}

#[inline]
fn begin_serialize<W: LqWriter>(writer: &mut W, float_type: Type) -> Result<(), LqError> {
    let length = match float_type {
        Type::F32 => 4,
        Type::F64 => 8,
    };
    writer.write_content_description(TYPE_FLOAT, &ContentDescription::new_self_length(length))
}

#[derive(PartialEq)]
enum Type {
    F32,
    F64,
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Float::F32(value) => write!(f, "Float32({})", value),
            Float::F64(value) => write!(f, "Float64({})", value),
        }
    }
}
