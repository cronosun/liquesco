use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::html::list_item;
use crate::html::span;
use minidom::Element;

// TODO: Remove

pub struct WAnchors;

impl<'a> BodyWriter<'a> for WAnchors {
    type T = TAnchors<'a>;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        let mut ul = Element::bare("ul");

        let master_element = list_item("Master anchor type", ctx.link(ctx.r#type.master()));
        ul.append_child(master_element);

        let anchor_type = list_item("Anchor type", ctx.link(ctx.r#type.anchor()));
        ul.append_child(anchor_type);

        if let Some(max_number_of_anchors) = ctx.r#type.max_anchors() {
            let max_anchors = list_item(
                "Maximum number of anchors",
                span(format!(
                    "{} (not including the master anchor)",
                    max_number_of_anchors
                )),
            );
            ul.append_child(max_anchors);
        }

        ul
    }
}

pub struct WReference;

impl<'a> BodyWriter<'a> for WReference {
    type T = TReference<'a>;

    fn write(_: &mut Context<Self::T>) -> Element {
        Element::bare("span")
    }
}
