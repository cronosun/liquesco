use liquesco_serialization::types::boolean::Bool;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;

use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::boolean::TBool;
use crate::types::enumeration::TEnum;
use crate::types::enumeration::Variant;
use crate::types::key_ref::TKeyRef;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::types::seq::SeqHeader;
use std::cmp::Ordering::Equal;

/// A range.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TRange<'a> {
    meta: Meta<'a>,
    element: TypeRef,
    inclusion: Inclusion,
    allow_empty: bool,
}

/// Defines whether start/end is included/excluded in the range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Inclusion {
    BothInclusive,
    StartInclusive,
    BothExclusive,
    EndInclusive,
    /// Exclusion / inclusion is provided in the data.
    Supplied,
}

impl<'a> TRange<'a> {
    pub fn new(element: TypeRef, inclusion: Inclusion, allow_empty: bool) -> Self {
        Self {
            meta: Meta::empty(),
            element,
            inclusion,
            allow_empty,
        }
    }

    /// Information about whether start and end are inclusive.
    pub fn inclusion(&self) -> Inclusion {
        self.inclusion
    }

    /// The type of the range element.
    pub fn element(&self) -> &TypeRef {
        &self.element
    }

    /// True if empty ranges are allowed.
    pub fn allow_empty(&self) -> bool {
        self.allow_empty
    }
}

impl Type for TRange<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq.length();

        let expected_number_of_items = if self.inclusion == Inclusion::Supplied {
            4
        } else {
            2
        };

        if number_of_items != expected_number_of_items {
            return LqError::err_new(format!(
                "The given range has a seq length of {}, we \
                 need a length of {} (start, end and maybe 2 more items with information about \
                 inclusion).",
                number_of_items, expected_number_of_items
            ));
        }

        // start
        let mut start_reader = context.reader().clone();
        context.validate(&self.element)?;
        // end
        let mut end_reader = context.reader().clone();
        context.validate(&self.element)?;

        let inclusive: (bool, bool) = match self.inclusion {
            Inclusion::Supplied => (
                Bool::de_serialize(context.reader())?,
                Bool::de_serialize(context.reader())?,
            ),
            Inclusion::BothInclusive => (true, true),
            Inclusion::StartInclusive => (true, false),
            Inclusion::BothExclusive => (false, false),
            Inclusion::EndInclusive => (false, true),
        };

        // Now compare start and end
        let cmp = context.compare(&self.element, &mut start_reader, &mut end_reader)?;
        match cmp {
            Ordering::Greater => LqError::err_new(
                "The given start (first element) is greater then \
                 given end (second element) in range. Start can never be greater than end.",
            ),
            Ordering::Equal => {
                if self.allow_empty {
                    Ok(())
                } else {
                    let ok = match inclusive {
                        (true, false) => false,
                        (true, true) => true,
                        (false, false) => false,
                        (false, true) => false,
                    };
                    if !ok {
                        LqError::err_new(format!("Start (first element) is equal to \
                        end (second element). \
                    This is not allowed according to the schema (see 'allow_empty'). Start \
                    inclusive {}, end inclusive {}.",
                        inclusive.0, inclusive.1))
                    } else {
                        Ok(())
                    }
                }
            }
            Ordering::Less => Ok(()),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let header1 = SeqHeader::de_serialize(r1)?;
        let header2 = SeqHeader::de_serialize(r2)?;

        if header1.length() != header2.length() {
            return LqError::err_new("Ranges with different length (cannot compare them).");
        }

        let with_inclusion = header1.length() == 4;

        let cmp1 = context.compare(&self.element, r1, r2)?;
        Ok(if cmp1 != Equal {
            cmp1
        } else {
            let cmp2 = context.compare(&self.element, r1, r2)?;
            if cmp2 != Equal {
                cmp2
            } else if with_inclusion {
                let cmp3 = Bool::de_serialize(r1)?.cmp(&Bool::de_serialize(r2)?);
                if cmp3 != Equal {
                    cmp3
                } else {
                    Bool::de_serialize(r1)?.cmp(&Bool::de_serialize(r2)?)
                }
            } else {
                Equal
            }
        })
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        if index == 0 {
            Some(&self.element)
        } else {
            None
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        if index == 0 {
            self.element = type_ref;
            Ok(())
        } else {
            LqError::err_new(format!("Range has no type at index {}", index))
        }
    }
}

impl WithMetadata for TRange<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TRange<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TRange<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let element_field = builder.add_unwrap(
            "range_element",
            TKeyRef::default().with_doc("The start and end type of the range."),
        );

        let inclusion_field = builder.add_unwrap(
            "inclusion",
            TEnum::default()
                .add_variant(Variant::new(
                    Identifier::try_from("both_inclusive").unwrap(),
                ))
                .add_variant(Variant::new(
                    Identifier::try_from("start_inclusive").unwrap(),
                ))
                .add_variant(Variant::new(
                    Identifier::try_from("both_exclusive").unwrap(),
                ))
                .add_variant(Variant::new(Identifier::try_from("end_inclusive").unwrap()))
                .add_variant(Variant::new(Identifier::try_from("supplied").unwrap()))
                .with_doc(
                    "Determines whether start and end are inclusive. There's one \
                     special value: 'Supplied'. When you choose this, the data has to contain \
                     the information whether start/end are inclusive or not.",
                ),
        );

        let allow_empty_field = builder.add_unwrap(
            "allow_empty",
            TBool::default()
            .with_doc("General rule is start < end. When start equals end it's \
            possible to construct empty ranges (depending on the inclusion). If this is false \
            it's not allowed to specify a range that's empty. You usually want this to be false."));

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("element").unwrap(),
                element_field,
            ))
            .add(Field::new(
                Identifier::try_from("inclusion").unwrap(),
                inclusion_field,
            ))
            .add(Field::new(
                Identifier::try_from("allow_empty").unwrap(),
                allow_empty_field,
            ))
            .with_doc("A sequence contains 0-n elements of the same type.")
    }
}
