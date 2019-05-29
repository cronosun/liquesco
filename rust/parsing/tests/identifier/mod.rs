use liquesco_parsing::yaml::parse_from_yaml_str;
use crate::builder::builder;
use crate::utils::{assert_err, assert_ok};
use liquesco_schema::core::Schema;
use liquesco_common::ine_range::U64IneRange;
use liquesco_schema::ascii::CodeRange;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::metadata::Meta;
use liquesco_schema::seq::TSeq;
use liquesco_schema::any_type::AnyType;

fn create_identifier_schema() -> impl Schema<'static> {
    let mut builder = builder();
    let ascii = builder.add(AnyType::Ascii(TAscii {
        meta: Meta::empty(),
        length: U64IneRange::try_new("", 0, 10).unwrap(),
        codes: CodeRange::try_new(97, 123).unwrap(),
    }));
    let identifier = builder.add(AnyType::Seq(TSeq::try_new(ascii, 1, 8).unwrap()));
    builder.finish(AnyType::Seq(TSeq::try_new(identifier, 1, 100).unwrap()))
}

#[test]
fn ok_1() {
    let schema = create_identifier_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("ok_identifier.yaml"),
    ))
}

#[test]
fn err_one_element_too_long() {
    let schema = create_identifier_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_segment_too_long.yaml"),
    ))
}
