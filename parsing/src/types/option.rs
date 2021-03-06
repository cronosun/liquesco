use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::option::TOption;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::option::Presence;

pub struct POption;

impl<'a> Parser<'a> for POption {
    type T = TOption<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        if value.as_ref().is_nothing() {
            Presence::serialize(writer, &Presence::Absent)?;
            Result::Ok(())
        } else {
            Presence::serialize(writer, &Presence::Present)?;
            context.parse(writer, r#type.r#type(), value)
        }
    }
}
