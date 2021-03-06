use crate::value::Seq;
use crate::value::Text;
use crate::value::TextValue;
use crate::value::Value;
use data_encoding::BASE64_NOPAD;
use data_encoding::HEXLOWER_PERMISSIVE;
use liquesco_common::decimal::Decimal;
use liquesco_common::error::LqError;
use liquesco_schema::identifier::Format;
use liquesco_schema::identifier::Identifier;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Write;
use std::ops::Deref;

/// Naming:
/// - require: Returns an error when conversion is not possible.
/// - to: converts to some optional.
pub trait Converter {
    /// Converts an identifier to a string suited for the text format.
    fn identifier_to_string(identifier: &Identifier, what_for: IdentifierType) -> String {
        match what_for {
            IdentifierType::StructField => identifier.to_string(Format::SnakeCase),
            IdentifierType::EnumIdentifier => identifier.to_string(Format::SnakeCase),
        }
    }

    fn string_to_identifier(value: &str, _: IdentifierType) -> Result<Identifier, LqError> {
        Ok(Identifier::try_from(value)?)
    }

    fn to_text<'a>(value: &'a Value<'a>) -> Option<&'a Text<'a>> {
        match value {
            Value::Text(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    fn require_text<'a>(value: &'a Value<'a>) -> Result<&'a Text<'a>, LqError> {
        require(Self::to_text(value), || {
            format!("Expecting to have a string; got {:?}", value)
        })
    }

    fn to_u128(value: &Value) -> Option<u128> {
        match value {
            Value::U64(value) => Option::Some(u128::from(*value)),
            Value::I64(value) => u128::try_from(*value).ok(),
            // TODO: Maybe also allow "MAX_8", "MAX_16", "MIN_8"...
            // TODO: Also accept hex encoding...
            Value::Text(text) => text.parse::<u128>().ok(),
            _ => Option::None,
        }
    }

    fn require_u128(value: &Value) -> Result<u128, LqError> {
        require(Self::to_u128(value), || {
            format!("Expecting an unsigned integer, got {:?}", value)
        })
    }

    fn to_bool(value: &Value) -> Option<bool> {
        match value {
            Value::Bool(value) => Option::Some(*value),
            Value::Text(value) => {
                let value_str: &str = &value;
                match value_str {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            }
            _ => Option::None,
        }
    }

    fn require_bool(value: &Value) -> Result<bool, LqError> {
        require(Self::to_bool(value), || {
            format!("Expecting a boolean, got {:?}", value)
        })
    }

    fn to_i128(value: &Value) -> Option<i128> {
        match value {
            Value::I64(value) => Option::Some(i128::from(*value)),
            Value::U64(value) => i128::try_from(*value).ok(),
            // TODO: Maybe also allow "MAX_8", "MAX_16", "MIN_8"...
            // TODO: Also accept hex encoding...
            Value::Text(text) => text.parse::<i128>().ok(),
            _ => Option::None,
        }
    }

    fn require_i128(value: &Value) -> Result<i128, LqError> {
        require(Self::to_i128(value), || {
            format!("Expecting a signed integer, got {:?}", value)
        })
    }

    fn to_f64(value: &Value) -> Option<f64> {
        match value {
            Value::F64(value) => Option::Some(*value),
            Value::Text(value) => value.parse::<f64>().ok(),
            _ => None,
        }
    }

    fn require_f64(value: &Value) -> Result<f64, LqError> {
        require(Self::to_f64(value), || {
            format!(
                "Expecting a float 64 (if this is an integer, try adding .0; e.g. 12 \
                 -> 12.0), got {:?}",
                value
            )
        })
    }

    fn to_f32(value: &Value) -> Option<f32> {
        match value {
            Value::F64(value) => {
                // accept when no precision is lost
                let f32_value = *value as f32;
                let f64_value = f32_value as f64;
                if (value - &f64_value).abs() == 0f64 {
                    Some(f32_value)
                } else {
                    None
                }
            }
            Value::Text(value) => value.parse::<f32>().ok(),
            _ => None,
        }
    }

    fn require_f32(value: &Value) -> Result<f32, LqError> {
        require(Self::to_f32(value), || {
            format!(
                "Expecting a float 32 (if this is an integer, try adding .0; e.g. 12 \
                 -> 12.0; of it looks like a float, make sure it can be represented as 32 bit \
                 float value without loosing precision), got {:?}",
                value
            )
        })
    }

    fn to_decimal(value: &Value) -> Option<Decimal> {
        // will also accept floats and ints
        if let Value::Text(text) = value {
            Decimal::try_from(text.deref()).ok()
        } else if let Some(float) = Self::to_f64(value) {
            let mut as_string = String::new();
            if let Ok(_) = write!(&mut as_string, "{}", float) {
                Decimal::try_from(as_string.as_str()).ok()
            } else {
                None
            }
        } else if let Some(int) = Self::to_i128(value) {
            Some(Decimal::from_parts(int, 0))
        } else {
            None
        }
    }

    fn require_decimal(value: &Value) -> Result<Decimal, LqError> {
        require(Self::to_decimal(value), || {
            format!(
                "Expecting a decimal value; Valid decimal values look like \
                 this: 12.23, 12, 5e-4 or 4e3. got {:?}",
                value
            )
        })
    }

    fn to_string_map<'a>(value: &'a Value<'a>) -> Option<HashMap<&'a str, &'a TextValue<'a>>> {
        if let Value::Seq(seq) = value {
            let mut result: HashMap<&'a str, &'a TextValue<'a>> = HashMap::with_capacity(seq.len());
            for entry in seq {
                if let Value::Seq(key_value) = &entry.value {
                    if key_value.len() == 2 {
                        if let Some(key) = Self::to_text(&key_value[0].value) {
                            result.insert(key, &key_value[1]);
                        } else {
                            return Option::None;
                        }
                    } else {
                        return Option::None;
                    }
                } else {
                    return Option::None;
                }
            }
            Option::Some(result)
        } else {
            Option::None
        }
    }

    fn require_string_map<'a>(
        value: &'a Value<'a>,
    ) -> Result<HashMap<&'a str, &'a TextValue<'a>>, LqError> {
        require(Self::to_string_map(value), || {
            format!(
                "Expecting to have a map with text keys (or a sequence with 0-n \
                 entries where each entry in turn is a sequence with 2 elements where the first \
                 of those 2 elements is a text), got {:?}",
                value
            )
        })
    }

    fn to_seq<'a>(value: &'a Value<'a>) -> Option<&'a Seq<'a>> {
        if let Value::Seq(value) = value {
            Some(value)
        } else {
            None
        }
    }

    fn require_seq<'a>(value: &'a Value<'a>) -> Result<&'a Seq<'a>, LqError> {
        require(Self::to_seq(value), || {
            format!(
                "Expecting to have a sequence (aka. list or vector). got {:?}",
                value
            )
        })
    }

    fn to_binary<'a>(value: &'a Value<'a>) -> Option<Cow<'a, [u8]>> {
        let maybe_vec = if let Some(cow) = Self::to_text(value) {
            let text: &str = &cow;
            let utf8_text: &[u8] = text.as_bytes();
            if text.starts_with("hex:") {
                // try to decode as hex (don't cate about case)
                HEXLOWER_PERMISSIVE.decode(&utf8_text[4..]).ok()
            } else {
                // try to decode that as base64
                BASE64_NOPAD.decode(utf8_text).ok()
            }
        } else {
            // sequence of bytes
            if let Some(seq) = Self::to_seq(value) {
                let number_of_elements = seq.len();
                let mut result = Vec::with_capacity(number_of_elements);
                for element in seq {
                    if let Some(element_as_number) = Self::to_u128(&element.value) {
                        if element_as_number <= std::u8::MAX as u128 {
                            result.push(element_as_number as u8);
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                Some(result)
            } else {
                None
            }
        };
        maybe_vec.map(|vec| Cow::Owned(vec))
    }

    fn require_binary<'a>(value: &'a Value<'a>) -> Result<Cow<'a, [u8]>, LqError> {
        require(Self::to_binary(value), || {
            format!(
                "Expecting to have binary data. Valid binary data is either base64 (string; \
                 no padding) or hex encoding (a string starting with 'hex:') or a sequence of \
                 numbers (0-255). Example (base 64): 'aGVsbG8'; Example (hex encoding): \
                 'hex:68656C6C6F'; Example (seq): [104, 101, 108, 108, 111]. got {:?}",
                value
            )
        })
    }
}

pub enum IdentifierType {
    StructField,
    EnumIdentifier,
}

fn require<T, Msg: FnOnce() -> String>(value: Option<T>, msg: Msg) -> Result<T, LqError> {
    if let Some(value) = value {
        Result::Ok(value)
    } else {
        Result::Err(LqError::new(msg()))
    }
}
