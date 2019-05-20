use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::doc_type::DocType;
use crate::identifier::Identifier;
use crate::option::TOption;
use crate::reference::TReference;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::Ordering as SeqOrdering;
use crate::seq::TSeq;
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_core::serialization::core::DeSerializer;
use liquesco_core::serialization::core::LqReader;
use liquesco_core::serialization::enumeration::EnumHeader;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::convert::TryFrom;

const MIN_VALUES: usize = 1;
const MAX_VALUES: usize = 32;
const MIN_VARIANTS: usize = 1;

/// Use a small vec with 5 items (should be enough for many cases)
type Variants<'a> = SmallVec<[Variant<'a>; 5]>;
type Values = SmallVec<[TypeRef; 3]>;

#[derive(new, Clone, Debug, Serialize, Deserialize, PartialEq, Hash)]
pub struct TEnum<'a> {
    variants: Variants<'a>,
}

#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Variant<'a> {
    /// Textual identifier of the variant.
    pub name: Identifier<'a>,

    /// The values this variant carries: This must contain > 0 items. It should only
    /// contain more then one item when you want to extend an existing schema and the value
    /// at index 0 is something you can't extend (a.g. not a struct).
    ///
    /// For variants without value, this is empty.
    #[serde(default)]
    #[new(value = "None")]
    values: Option<Values>,
}

impl<'a> Variant<'a> {
    pub fn name(&self) -> &Identifier<'a> {
        &self.name
    }

    pub fn values(&self) -> &[TypeRef] {
        match &self.values {
            Option::None => &[],
            Option::Some(values) => values,
        }
    }

    pub fn add_value(mut self, value: TypeRef) -> Self {
        if let None = self.values {
            self.values = Some(Values::default());
        }
        let borrowed_self: &mut Self = &mut self;
        if let Some(values) = &mut borrowed_self.values {
            values.push(value);
        }
        self
    }
}

impl<'a> Default for TEnum<'a> {
    fn default() -> Self {
        Self {
            variants: Variants::new(),
        }
    }
}

impl<'a> TEnum<'a> {
    pub fn variants(&self) -> &[Variant<'a>] {
        &self.variants
    }

    pub fn add(mut self, variant: Variant<'a>) -> Self {
        self.variants.push(variant);
        self
    }

    pub fn variant_by_id<'b>(&self, id: &Identifier<'b>) -> Option<(u32, &Variant<'a>)> {
        // maybe better use a map for the variants?
        let mut ordinal: u32 = 0;
        for variant in &self.variants {
            if variant.name.is_equal(id) {
                return Option::Some((ordinal, variant));
            }
            ordinal = ordinal + 1;
        }
        Option::None
    }
}

impl<'a> Type for TEnum<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let enum_header = EnumHeader::de_serialize(context.reader())?;
        let number_of_values = enum_header.number_of_values();
        let ordinal = enum_header.ordinal();

        let number_of_variants = self.variants.len();

        let usize_ordinal = usize::try_from(ordinal)?;
        if usize_ordinal >= number_of_variants {
            return LqError::err_new(format!(
                "Got ordinal value {:?} for enum. \
                 There's no such variant defined for that ordinal value in \
                 the schema.",
                ordinal
            ));
        }
        let variant = &self.variants[usize_ordinal];

        let usize_number_of_values = usize::try_from(number_of_values)?;
        let schema_number_of_values = variant.values().len();
        if context.config().no_extension() && (schema_number_of_values != usize_number_of_values) {
            return LqError::err_new(format!(
                "Error processing enum variant {:?} (ordinal \
                 {:?}); strict mode: Schema expects {:?} values - have {:?} values in \
                 data.",
                variant.name(),
                ordinal,
                schema_number_of_values,
                usize_number_of_values
            ));
        } else if usize_number_of_values < schema_number_of_values {
            return LqError::err_new(format!(
                "Error processing enum variant {:?} (ordinal \
                 {:?}): Schema expects at least {:?} values - have {:?} values in \
                 data.",
                variant.name(),
                ordinal,
                schema_number_of_values,
                usize_number_of_values
            ));
        }

        let to_skip = usize_number_of_values - schema_number_of_values;

        // validate each element
        for r#type in variant.values() {
            context.validate(*r#type)?;
        }

        if to_skip > 0 {
            context.reader().skip_n_values(to_skip)?;
        }

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let header1 = EnumHeader::de_serialize(r1)?;
        let header2 = EnumHeader::de_serialize(r2)?;

        // compare ordinals
        let ordinal_cmp = header1.ordinal().cmp(&header2.ordinal());
        if ordinal_cmp != Ordering::Equal {
            Result::Ok(ordinal_cmp)
        } else {
            // same ordinal, we also have to compare content: but important: We do only compare
            // the values that are defined in the schema. Why? If we'd compare more we could
            // just add some arbitrary data and thus add data that's unique (according to the
            // values in the schema) into a a sequence with a unique constraint.

            let ordinal = header1.ordinal();
            let usize_ordinal = usize::try_from(ordinal)?;
            let number_of_variants = self.variants.len();
            if usize_ordinal >= number_of_variants {
                return LqError::err_new(format!(
                    "Got ordinal value {:?} for enum. \
                     There's no such variant defined for that ordinal value in \
                     the schema.",
                    ordinal
                ));
            }

            let variant = &self.variants[usize_ordinal];
            let mut num_read: u32 = 0;
            for r#type in variant.values() {
                let cmp = context.compare(*r#type, r1, r2)?;
                num_read = num_read + 1;
                if cmp != Ordering::Equal {
                    // no need to finish to the end (see contract)
                    return Result::Ok(cmp);
                }
            }

            // equal: read the rest (see contract)
            // it's very important that we finish reading to the end (see contract)
            let finish_reading =
                |header: EnumHeader, reader: &mut LqReader, num_read: u32| -> Result<(), LqError> {
                    let len = header.number_of_values();
                    if len > num_read {
                        let missing = len - num_read;
                        reader.skip_n_values_u32(missing)
                    } else {
                        Result::Ok(())
                    }
                };

            finish_reading(header1, r1, num_read)?;
            finish_reading(header2, r2, num_read)?;

            Result::Ok(Ordering::Equal)
        }
    }
}

fn build_variant_schema<B>(builder: &mut B) -> TypeRef
where
    B: SchemaBuilder,
{
    let field_name = Identifier::build_schema(builder);

    let single_value = builder.add(DocType::from(TReference));
    let values = builder.add(DocType::from(TSeq {
        element: single_value,
        length: U32IneRange::try_new(MIN_VALUES as u32, MAX_VALUES as u32).unwrap(),
        ordering: SeqOrdering::None,
        multiple_of: None,
    }));
    let field_values = builder.add(DocType::from(TOption::new(values)));

    builder.add(DocType::from(
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("name").unwrap(),
                field_name,
            ))
            .add(Field::new(
                Identifier::try_from("values").unwrap(),
                field_values,
            )),
    ))
}

impl<'a> BaseTypeSchemaBuilder for TEnum<'a> {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let variant = build_variant_schema(builder);
        let field_variants = builder.add(DocType::from(TSeq {
            element: variant,
            length: U32IneRange::try_new(MIN_VARIANTS as u32, std::u32::MAX).unwrap(),
            ordering: SeqOrdering::None,
            multiple_of: None,
        }));

        DocType::from(TStruct::default().add(Field::new(
            Identifier::try_from("variants").unwrap(),
            field_variants,
        )))
    }
}