use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::range::{Inclusion, TRange};
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::boolean::Bool;
use liquesco_serialization::types::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PRange;

impl<'a> Parser<'a> for PRange {
    type T = TRange<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let sequence = C::TConverter::require_seq(&value.value)?;
        let supplied_inclusion = r#type.inclusion() == Inclusion::Supplied;
        let number_of_fields = if supplied_inclusion { 4 } else { 2 };

        if sequence.len() != number_of_fields {
            return LqError::err_new(format!("Range parsing: Got a sequence (this is \
            correct) but the sequence has a length of {}. I need a length of {}: [start, end, \
            start included, end included] or just [start, end] (when schema provides inclusion).",
                                            sequence.len(), number_of_fields));
        }

        let u32_number_of_fields = u32::try_from(number_of_fields)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_fields))?;

        context.parse(writer, r#type.element(), &sequence[0])?;
        context.parse(writer, r#type.element(), &sequence[1])?;

        if supplied_inclusion {
            let value = C::TConverter::require_bool(&sequence[2].value)?;
            Bool::serialize(writer, &value)?;
            let value = C::TConverter::require_bool(&sequence[3].value)?;
            Bool::serialize(writer, &value)?;
        }

        Ok(())
    }
}
