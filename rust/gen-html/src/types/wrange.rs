use liquesco_schema::uint::TUInt;
use liquesco_schema::sint::TSInt;
use crate::html::span;
use crate::html::list_item;
use minidom::Element;
use crate::body_writer::Context;
use crate::body_writer::BodyWriter;
use liquesco_schema::range::{TRange, Inclusion};
use crate::reference::Reference;

pub struct WRange;

impl BodyWriter for WRange {
    type T = TRange;

    fn write(ctx : &mut Context<Self::T>) -> Element {
       let mut ul = Element::bare("ul");

        let type_info = ctx.schema.type_info(ctx.r#type.element());
        let link = Reference {
            type_info : &type_info,
            names : &mut ctx.names,
        }.link();

        let range_element = list_item(
            "Range element",
            link
        );
        ul.append_child(range_element);

        let inclusion : (&str, &str) = match ctx.r#type.inclusion() {
            Inclusion::BothInclusive => ("Inclusive", "Inclusive"),
            Inclusion::StartInclusive => ("Inclusive", "Exclusive"),
            Inclusion::BothExclusive => ("Exclusive", "Exclusive"),
            Inclusion::EndInclusive => ("Exclusive", "Inclusive"),
            Inclusion::Supplied => ("Supplied (by data)", "Supplied (by data)"),
        };

        let start_inclusion = list_item(
            "Start inclusive",
            span(inclusion.0)
        );
        ul.append_child(start_inclusion);

        let end_inclusion = list_item(
            "End inclusive",
            span(inclusion.1)
        );
        ul.append_child(end_inclusion);

        ul
    }
}
