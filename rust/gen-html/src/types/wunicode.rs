use liquesco_schema::unicode::TUnicode;
use crate::html::span;
use crate::html::list_item;
use minidom::Element;
use crate::body_writer::Context;
use crate::body_writer::BodyWriter;
use liquesco_schema::unicode;

pub struct WUnicode;

impl BodyWriter for WUnicode {
    type T = TUnicode;

    fn write(ctx : &mut Context<Self::T>) -> Element {
       let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum length (inclusive)",
            span(format!("{value}", value = ctx.r#type.length().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum length (inclusive)",
            span(format!("{value}", value = ctx.r#type.length().end())),
        );
        ul.append_child(max_len);

        let length_str = match ctx.r#type.length_type() {
            unicode::LengthType::Byte => "Number of bytes (actual text length depends on encoding)",
            unicode::LengthType::Utf8Byte => {
                "Number of UTF-8 bytes (needs to compute the length when encoding is not UTF-8)"
            }
            unicode::LengthType::ScalarValue => {
                "Unicode scalar values (this is not grapheme clusters)"
            }
        };
        ul.append_child(list_item("Length type", span(length_str)));

        ul
    }
}