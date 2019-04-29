use crate::serialization::core::LqError;
use crate::serialization::value::EnumValue;
use crate::serialization::value::Value;
use crate::serialization::value::ValueList;
use crate::serialization::value::ValueRef;
use std::convert::TryFrom;

use std::borrow::Cow;

impl From<bool> for Value<'static> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value<'static> {
    fn from(value: String) -> Self {
        Value::Utf8(Cow::Owned(value))
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::Utf8(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Self {
        Value::Binary(Cow::Borrowed(value))
    }
}

impl<'a, T: Into<Value<'a>>> From<Option<T>> for Value<'a> {
    fn from(value: Option<T>) -> Self {
        match value {
            Option::Some(value) => {
                Value::Option(Option::Some(ValueRef::Boxed(Box::new(value.into()))))
            }
            Option::None => Value::Option(Option::None),
        }
    }
}

impl<'a, T: Into<Value<'a>>> From<Vec<T>> for Value<'a> {
    fn from(value: Vec<T>) -> Self {
        let mut vec: Vec<Value<'a>> = Vec::with_capacity(value.len());
        for item in value {
            vec.push(item.into());
        }
        Value::List(ValueList::Owned(vec))
    }
}

impl From<u64> for Value<'static> {
    fn from(value: u64) -> Self {
        Value::UInt(value)
    }
}

impl From<i64> for Value<'static> {
    fn from(value: i64) -> Self {
        Value::SInt(value)
    }
}

impl From<u32> for Value<'static> {
    fn from(value: u32) -> Self {
        Value::UInt(u64::from(value))
    }
}

impl From<i32> for Value<'static> {
    fn from(value: i32) -> Self {
        Value::SInt(i64::from(value))
    }
}

impl<'a> TryFrom<&'a Value<'a>> for bool {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Bool(boolean) = value {
            Result::Ok(*boolean)
        } else {
            invalid_type("bool", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for &'a [u8] {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Binary(bin) = value {
            Result::Ok(bin)
        } else {
            invalid_type("binary", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for Option<ValueRef<'a>> {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Option(opt) = value {
            Result::Ok(opt.clone())
        } else {
            invalid_type("optional", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for &'a [Value<'a>] {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::List(list) = value {
            Result::Ok(list)
        } else {
            invalid_type("list", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for &'a EnumValue<'a> {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Enum(enum_value) = value {
            Result::Ok(enum_value)
        } else {
            invalid_type("enum", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for &'a str {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Utf8(utf8) = value {
            Result::Ok(utf8)
        } else {
            invalid_type("utf8", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for u64 {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::UInt(uint) = value {
            Result::Ok(*uint)
        } else {
            invalid_type("unsigned int", value)
        }
    }
}

impl<'a> TryFrom<&'a Value<'a>> for i64 {
    type Error = LqError;

    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        if let Value::SInt(sint) = value {
            Result::Ok(*sint)
        } else {
            invalid_type("signed int", value)
        }
    }
}

fn invalid_type<Ok>(type_name: &'static str, value: &Value) -> Result<Ok, LqError> {
    LqError::err_new(format!(
        "Cannot convert (try_into) value, wrong type. Expecting {:?} have {:?}",
        type_name, value
    ))
}