use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::core::Schema;
use liquesco_schema::doc_type::DocType;
use liquesco_schema::option::TOption;
use liquesco_schema::seq::TSeq;
use liquesco_schema::structure::Field;
use liquesco_schema::structure::TStruct;
use liquesco_schema::uint::TUInt;
use liquesco_schema::unicode::LengthType;
use liquesco_schema::unicode::TUnicode;

fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    // a structure: a person
    let field_first_name = builder.add(AnyType::Unicode(DocType::from(
        TUnicode::try_new(1, 100, LengthType::Byte).unwrap(),
    )));
    let field_last_name = builder.add(AnyType::Unicode(DocType::from(
        TUnicode::try_new(1, 100, LengthType::Byte).unwrap(),
    )));
    let field_year_born = builder.add(AnyType::UInt(DocType::from(
        TUInt::try_new(1000, 3000).unwrap(),
    )));
    let email = builder.add(AnyType::Ascii(DocType::from(
        TAscii::try_new(1, 100, 0, 127).unwrap(),
    )));
    let field_email = builder.add(AnyType::Option(DocType::from(TOption::new(email))));

    let struct_type = TStruct::default()
        .add(Field::new(id("first_name"), field_first_name))
        .add(Field::new(id("last_name"), field_last_name))
        .add(Field::new(id("year_born"), field_year_born))
        .add(Field::new(id("email"), field_email));
    let doc_struct_type: DocType<'static, TStruct> = struct_type.into();

    let structure = builder.add(doc_struct_type);

    // people (structure) within a sequence
    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(structure, 1, 20).unwrap(),
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema2/working1.yaml"),
    ))
}

#[test]
fn unused_field() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema2/unused_field.yaml"),
    ))
}