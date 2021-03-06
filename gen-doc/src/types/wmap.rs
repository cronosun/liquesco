use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::{Common, TxtSorting};
use liquesco_common::error::LqError;
use liquesco_schema::types::map::{Sorting, TMap};
use std::marker::PhantomData;

pub struct WMap<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WMap<'a> {
    type T = TMap<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(vec![
            ctx.named_link_to_type("Key type", typ.key())?,
            ctx.named_link_to_type("Value type", typ.value())?,
            Row::association_with_text(
                "Sorting",
                Common::txt_sorting(match typ.sorting() {
                    Sorting::Ascending => TxtSorting::Ascending,
                    Sorting::Descending => TxtSorting::Descending,
                }),
            ),
            Row::association_with_text(
                "Min length (inclusive)",
                Common::fmt_u32(*typ.length().start()),
            ),
            Row::association_with_text(
                "Max length (inclusive)",
                Common::fmt_u32(*typ.length().end()),
            ),
            Row::association_with_text(
                "Anchors (can be referenced)",
                Common::fmt_bool_yes_no(typ.anchors()),
            ),
        ])
    }
}
