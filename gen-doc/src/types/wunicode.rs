use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::unicode;
use liquesco_schema::types::unicode::TUnicode;
use std::marker::PhantomData;

pub struct WUnicode<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WUnicode<'a> {
    type T = TUnicode<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let length_str = match typ.length_type() {
            unicode::LengthType::Byte => "Number of bytes (actual text length depends on encoding)",
            unicode::LengthType::Utf8Byte => {
                "Number of UTF-8 bytes (needs to compute the length when encoding is not UTF-8)"
            }
            unicode::LengthType::ScalarValue => {
                "Unicode scalar values (this is not grapheme clusters)"
            }
        };

        Ok(vec![
            Row::association_with_text("Length type", length_str),
            Row::association_with_text(
                "Minimum length (inclusive)",
                Common::fmt_u64(*typ.length().start()),
            ),
            Row::association_with_text(
                "Maximum length (inclusive)",
                Common::fmt_u64(*typ.length().end()),
            ),
        ])
    }
}
