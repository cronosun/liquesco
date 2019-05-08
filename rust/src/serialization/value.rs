use crate::serialization::tfloat::Float;
use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::major_types::TYPE_BINARY;
use crate::serialization::major_types::TYPE_BOOL_FALSE;
use crate::serialization::major_types::TYPE_BOOL_TRUE;
use crate::serialization::major_types::TYPE_ENUM_0;
use crate::serialization::major_types::TYPE_ENUM_1;
use crate::serialization::major_types::TYPE_ENUM_2;
use crate::serialization::major_types::TYPE_ENUM_N;
use crate::serialization::major_types::TYPE_LIST;
use crate::serialization::major_types::TYPE_OPTION;
use crate::serialization::major_types::TYPE_SINT;
use crate::serialization::major_types::TYPE_UINT;
use crate::serialization::major_types::TYPE_UNICODE;
use crate::serialization::major_types::TYPE_FLOAT;
use crate::serialization::tbinary::Binary;
use crate::serialization::tenum::EnumHeader;
use crate::serialization::tseq::SeqHeader;
use crate::serialization::toption::Presence;
use crate::serialization::tsint::SInt64;
use crate::serialization::tuint::UInt64;
use crate::serialization::tunicode::Unicode;
use crate::serialization::tbool::Bool;
use std::convert::TryFrom;
use std::ops::Deref;

use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Value<'a> {
    Bool(bool),
    Unicode(Cow<'a, str>),
    Binary(Cow<'a, [u8]>),
    Option(Option<ValueRef<'a>>),
    List(ValueList<'a>),
    Enum(EnumValue<'a>),
    UInt(u64),
    SInt(i64),
    Float(Float),
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ValueRef<'a> {
    Borrowed(&'a Value<'a>),
    Boxed(Box<Value<'a>>),
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ValueList<'a> {
    Owned(Vec<Value<'a>>),
    Borrowed(&'a [Value<'a>]),
    Empty,
}

const EMPTY_VALUE_VEC: &[Value<'static>] = &[];

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct EnumValue<'a> {
    ordinal: u32,
    values: ValueList<'a>,
}

impl<'a> EnumValue<'a> {
    pub fn new_no_value(ordinal: u32) -> EnumValue<'static> {
        EnumValue {
            ordinal,
            values: ValueList::Empty,
        }
    }

    pub fn new(ordinal: u32, value: Value) -> EnumValue {
        EnumValue {
            ordinal,
            values: ValueList::Owned(vec![value]),
        }
    }

    pub fn new_values(ordinal: u32, values: ValueList) -> EnumValue {
        EnumValue { ordinal, values }
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub fn values(&self) -> &ValueList<'a> {
        &self.values
    }
}

impl<'a> From<Value<'a>> for ValueRef<'a> {
    fn from(value: Value<'a>) -> Self {
        ValueRef::Boxed(Box::new(value))
    }
}

impl<'a> From<&'a Value<'a>> for ValueRef<'a> {
    fn from(value: &'a Value<'a>) -> Self {
        ValueRef::Borrowed(value)
    }
}

impl<'a> Deref for ValueList<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        match self {
            ValueList::Borrowed(value) => value,
            ValueList::Owned(value) => value,
            ValueList::Empty => EMPTY_VALUE_VEC,
        }
    }
}

impl<'a> Deref for ValueRef<'a> {
    type Target = Value<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            ValueRef::Borrowed(value) => *value,
            ValueRef::Boxed(value) => &value,
        }
    }
}

impl<'a> DeSerializer<'a> for Value<'a> {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let major_type = reader.peek_header()?.major_type();
        let value = match major_type {
            TYPE_BOOL_FALSE | TYPE_BOOL_TRUE => Value::Bool(Bool::de_serialize(reader)?),
            TYPE_OPTION => {
                let presence = Presence::de_serialize(reader)?;
                match presence {
                    Presence::Present => Value::Option(Option::Some(ValueRef::Boxed(Box::new(
                        Value::de_serialize(reader)?,
                    )))),
                    Presence::Absent => Value::Option(Option::None),
                }
            }
            TYPE_LIST => {
                let list_data = SeqHeader::de_serialize(reader)?;
                let length = list_data.length();
                if length == 0 {
                    Value::List(ValueList::Empty)
                } else {
                    let usize_length = try_from_int_result(usize::try_from(length))?;
                    let mut vec = Vec::with_capacity(usize_length);
                    for _ in 0..length {
                        vec.push(Value::de_serialize(reader)?);
                    }
                    Value::List(ValueList::Owned(vec))
                }
            }
            TYPE_BINARY => {
                let bin = Binary::de_serialize(reader)?;
                Value::Binary(Cow::Borrowed(bin))
            }
            TYPE_UNICODE => {
                let string = Unicode::de_serialize(reader)?;
                Value::Unicode(Cow::Borrowed(string))
            }
            TYPE_ENUM_0 | TYPE_ENUM_1 | TYPE_ENUM_2 | TYPE_ENUM_N => {
                let enum_header = EnumHeader::de_serialize(reader)?;
                let number_of_values = enum_header.number_of_values();
                if number_of_values > 0 {
                    // de-serialize data
                    let usize_number_of_values =
                        try_from_int_result(usize::try_from(number_of_values))?;
                    let mut values = Vec::with_capacity(usize_number_of_values);
                    for _ in 0..number_of_values {
                        values.push(Value::de_serialize(reader)?);
                    }
                    Value::Enum(EnumValue {
                        ordinal: enum_header.ordinal(),
                        values: ValueList::Owned(values),
                    })
                } else {
                    Value::Enum(EnumValue {
                        ordinal: enum_header.ordinal(),
                        values: ValueList::Empty,
                    })
                }
            }
            TYPE_UINT => {
                let value = UInt64::de_serialize(reader)?;
                Value::UInt(value)
            }
            TYPE_SINT => {
                let value = SInt64::de_serialize(reader)?;
                Value::SInt(value)
            },
            TYPE_FLOAT => {
                let value = Float::de_serialize(reader)?;
                Value::Float(value)
            }
            _ => {
                return LqError::err_new(format!("Unknown type {:?}", major_type));
            }
        };
        Result::Ok(value)
    }
}

impl<'a> Serializer for Value<'a> {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Value::Bool(value) => Bool::serialize(writer, value),
            Value::Option(value) => match value {
                Option::Some(present) => {
                    Presence::serialize(writer, &Presence::Present)?;
                    Value::serialize(writer, present)
                }
                Option::None => Presence::serialize(writer, &Presence::Absent),
            },
            Value::List(value) => {
                let len = value.len();
                let u32_len = try_from_int_result(u32::try_from(len))?;
                let list_data = SeqHeader::new(u32_len);
                SeqHeader::serialize(writer, &list_data)?;
                for item in value.deref() {
                    Value::serialize(writer, item)?;
                }
                Result::Ok(())
            }
            Value::Binary(value) => Binary::serialize(writer, value),
            Value::Unicode(value) => Unicode::serialize(writer, value),
            Value::Enum(value) => {
                let number_of_items = (&(value.values)).len();
                let u32_number_of_items = try_from_int_result(u32::try_from(number_of_items))?;
                let enum_header = EnumHeader::new(value.ordinal(), u32_number_of_items);
                EnumHeader::serialize(writer, &enum_header)?;

                // write values
                if number_of_items > 0 {
                    for value in value.values.deref() {
                        Value::serialize(writer, value)?;
                    }
                }
                Result::Ok(())
            }
            Value::UInt(value) => UInt64::serialize(writer, value),
            Value::SInt(value) => SInt64::serialize(writer, value),
            Value::Float(value) => Float::serialize(writer, value)
        }
    }
}
