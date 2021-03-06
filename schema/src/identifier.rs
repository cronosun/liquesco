use crate::core::TypeRef;
use crate::metadata::{Meta, MetadataSetter};
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use crate::types::ascii::{CodeRange, TAscii};
use crate::types::seq::TSeq;
use core::convert::TryFrom;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U64IneRange;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::{LqReader, LqWriter, Serializer};
use liquesco_serialization::types::seq::SeqHeader;
use liquesco_serialization::types::unicode::Unicode;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

const SEGMENT_MIN_LEN: usize = 1;
const SEGMENT_MAX_LEN: usize = 30;
const MIN_NUMBER_OF_SEGMENTS: usize = 1;
const MAX_NUMBER_OF_SEGMENTS: usize = 12;

/// A single segment within an identifier.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Segment<'a>(Cow<'a, str>);

/// The identifier is used to identify various parts in the system. It's very simple
/// and only supports lowercase ASCII characters and numbers (so it's simple to
/// convert that to identifiers in the target language when generating code from the
/// schema).
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Identifier<'a>(Vec<Segment<'a>>);

impl<'a> Deref for Identifier<'a> {
    type Target = [Segment<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}
impl<'a> Deref for Segment<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Identifier<'a> {
    /// Creates a new identifier that owns the string.
    pub fn new_owned(value: &str) -> Result<Identifier<'static>, LqError> {
        let splits = value.split('_');
        let mut segments = Vec::new();
        for split in splits {
            segments.push(Segment::try_from(split.to_string())?);
        }
        let number_of_segments = segments.len();
        Identifier::validate_number_of_segments(number_of_segments)?;
        Result::Ok(Identifier(segments))
    }

    pub fn segments(&self) -> &[Segment<'a>] {
        &self.0.as_slice()
    }

    pub fn to_string(&self, format: Format) -> String {
        match format {
            Format::SnakeCase => self
                .segments()
                .iter()
                .map(|segment| {
                    let string: &str = &segment.0;
                    string
                })
                .collect::<Vec<&str>>()
                .join("_"),
        }
    }

    pub fn append(&mut self, segment: Segment<'a>) -> Result<(), LqError> {
        if self.segments().len() + 1 > MAX_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "Cannot add another segment to identifier {:?}. Max number of segments reached.",
                self
            ))
        } else {
            self.0.push(segment);
            Ok(())
        }
    }

    pub fn into_owned(self) -> Identifier<'static> {
        let mut new_segments = Vec::with_capacity(self.0.len());
        for segment in self.0 {
            new_segments.push(segment.into_owned());
        }
        Identifier(new_segments)
    }

    /// Never make this public. It's only to be used internally.
    fn new_no_validation(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(value) => {
                // borrowed version
                let splits = value.split('_');
                let mut segments = Vec::new();
                for split in splits {
                    segments.push(Segment(Cow::Borrowed(split)));
                }
                Identifier(segments)
            }
            Cow::Owned(owned) => {
                let value: String = owned;
                let splits = value.split('_');
                let mut segments = Vec::new();
                for split in splits {
                    segments.push(Segment(Cow::Owned(split.to_string())));
                }
                Identifier(segments)
            }
        }
    }
}

impl BuildsOwnSchema for Identifier<'_> {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder<'static>,
    {
        let mut code_range = CodeRange::try_new(48, 57 + 1).unwrap();
        code_range.add(97, 122 + 1).unwrap();
        let segment_ref = builder.add_unwrap(
            "segment",
            TAscii::new(
                U64IneRange::try_new(
                    "Segment len",
                    SEGMENT_MIN_LEN as u64,
                    SEGMENT_MAX_LEN as u64,
                )
                .unwrap(),
                code_range,
            )
            .with_doc(
                "A single segment of an identifier. \
                 An identifier can only contain certain ASCII characters and is limited in length.",
            ),
        );
        let meta = Meta {
            doc: Some(Cow::Owned(format!(
                "An identifier identifies something in the system. An \
                 identifier is composed of {min}-{max} segments. Each segment is composed of ASCII \
                 characters (see segment for details what characters are allowed and about min/max \
                 length). These strict constraints allow simple conversions of identifiers to \
                 identifiers of the target system (e.g. Java class names, Rust trait names, Dart \
                 class names, ...).",
                min = MIN_NUMBER_OF_SEGMENTS,
                max = MAX_NUMBER_OF_SEGMENTS
            ))),
            implements: None,
        };
        builder.add_unwrap(
            "identifier",
            TSeq::try_new(
                segment_ref,
                MIN_NUMBER_OF_SEGMENTS as u32,
                MAX_NUMBER_OF_SEGMENTS as u32,
            )
            .unwrap()
            .with_meta(meta),
        )
    }
}

/// How to format the identifier.
pub enum Format {
    /// Snake case format (e.g. `my_identifier`, `number_of_segments`).
    SnakeCase,
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = LqError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let splits = value.split('_');
        let mut segments = Vec::new();
        for split in splits {
            segments.push(Segment::try_from(split)?);
        }
        let number_of_segments = segments.len();
        Identifier::validate_number_of_segments(number_of_segments)?;
        Result::Ok(Identifier(segments))
    }
}

impl<'a> TryFrom<&'a str> for Segment<'a> {
    type Error = LqError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Segment::validate(value)?;
        Result::Ok(Segment(Cow::Borrowed(value)))
    }
}

impl<'a> TryFrom<String> for Segment<'a> {
    type Error = LqError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Segment::validate(&value)?;
        Result::Ok(Segment(Cow::Owned(value)))
    }
}

impl<'a> Segment<'a> {
    fn validate(value: &str) -> Result<(), LqError> {
        let num_bytes = value.len();
        if num_bytes < SEGMENT_MIN_LEN {
            return LqError::err_new(format!(
                "Segment in identifier is too short (min {:?}), got: {:?}",
                SEGMENT_MIN_LEN, value
            ));
        } else if num_bytes > SEGMENT_MAX_LEN {
            return LqError::err_new(format!(
                "Segment in identifier is too long (max {:?}), got: {:?}",
                SEGMENT_MAX_LEN, value
            ));
        }
        // iterating bytes is OK, since we only accept ASCII anyway
        for byte_char in value.bytes() {
            let is_valid =
                (byte_char >= 97 && byte_char <= 122) || (byte_char >= 48 && byte_char <= 57);
            if !is_valid {
                return LqError::err_new(format!(
                    "The given segment in identifier is not valid. Only supports ASCII a-z \
                     (lower case) and 0-9; got: {:?}",
                    value
                ));
            }
        }
        Result::Ok(())
    }

    pub fn into_owned(self) -> Segment<'static> {
        match self.0 {
            Cow::Owned(item) => Segment(Cow::Owned(item)),
            Cow::Borrowed(item) => Segment(Cow::Owned(item.to_string())),
        }
    }
}

impl<'a> Identifier<'a> {
    fn validate_number_of_segments(number: usize) -> Result<(), LqError> {
        if number < MIN_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "An identifier needs at least {:?} segment(s); \
                 got {:?} segments",
                MIN_NUMBER_OF_SEGMENTS, number
            ))
        } else if number > MAX_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "An identifier can have at max {:?} segments; \
                 got {:?} segments",
                MAX_NUMBER_OF_SEGMENTS, number
            ))
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> DeSerializer<'a> for Identifier<'a> {
    type Item = Self;

    fn de_serialize<T: LqReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let list_header = SeqHeader::de_serialize(reader)?;
        let number_of_segments = list_header.length();
        let usize_number_of_segments = usize::try_from(number_of_segments)?;
        Identifier::validate_number_of_segments(usize_number_of_segments)?;
        let mut segments = Vec::with_capacity(usize_number_of_segments);
        for _ in 0..number_of_segments {
            let segment_str = Unicode::de_serialize(reader)?;
            segments.push(Segment::try_from(segment_str)?);
        }
        Result::Ok(Identifier(segments))
    }
}

impl<'a> Serializer for Identifier<'a> {
    type Item = Self;

    fn serialize<T: LqWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let number_of_segments = item.len();
        let u32_number_of_segments = u32::try_from(number_of_segments)?;
        let list_header = SeqHeader::new(u32_number_of_segments);
        SeqHeader::serialize(writer, &list_header)?;

        for segment in item.segments() {
            Unicode::serialize(writer, &segment)?;
        }
        Result::Ok(())
    }
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Id({})", self.to_string(Format::SnakeCase))
    }
}

/// Same as `Identifier` but internally stored as string.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StrIdentifier<'a>(Cow<'a, str>);

impl<'a> StrIdentifier<'a> {
    pub fn as_string(&self) -> &str {
        &self.0
    }
}

impl<'a> TryFrom<Cow<'a, str>> for StrIdentifier<'a> {
    type Error = LqError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let splits = value.split('_');
        let mut number_of_segments = 0;
        for split in splits {
            Segment::validate(split)?;
            number_of_segments += 1;
        }
        Identifier::validate_number_of_segments(number_of_segments)?;
        Result::Ok(StrIdentifier(value))
    }
}

/// We can now convert without try, since `StrIdentifier' has already been validated.
impl<'a> From<StrIdentifier<'a>> for Identifier<'a> {
    fn from(str_id: StrIdentifier<'a>) -> Self {
        Identifier::new_no_validation(str_id.0)
    }
}

impl<'a> Display for StrIdentifier<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Id({})", &self.0)
    }
}

#[test]
#[cfg(test)]
fn test_valid_identifiers() {
    Identifier::try_from("some_identifier").unwrap();
    // even segments starting with a number is OK (this is not allowed in some languages)
    Identifier::try_from("3_some_identifier").unwrap();
    // minimum possible
    Identifier::try_from("a").unwrap();
    // maximum possible
    Identifier::try_from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_\
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_\
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_\
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_\
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
}

#[test]
#[should_panic]
#[cfg(test)]
fn test_too_short() {
    Identifier::try_from("").unwrap();
}

#[test]
#[should_panic]
#[cfg(test)]
fn test_invalid_character() {
    Identifier::try_from("good_éé").unwrap();
}

#[test]
#[should_panic]
#[cfg(test)]
fn test_segment_too_long() {
    Identifier::try_from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
}

#[test]
#[should_panic]
#[cfg(test)]
fn too_many_segments() {
    Identifier::try_from("a_a_a_a_a_a_a_a_a_a_a_a_a").unwrap();
}
