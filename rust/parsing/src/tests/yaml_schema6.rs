use crate::tests::builder::builder;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Schema;
use liquesco_schema::range::{Inclusion, TRange};
use liquesco_schema::seq::TSeq;
use liquesco_schema::uint::TUInt;
use liquesco_schema::metadata::Meta;

fn create_schema_given_inclusion() -> impl Schema<'static> {
    let mut builder = builder();

    let range_element = builder.add(AnyType::UInt(
        TUInt::try_new(5, 150).unwrap(),
    ));

    let range_value = TRange {
        meta : Meta::empty(),
        element: range_element,
        inclusion: Inclusion::StartInclusive,
        allow_empty: false,
    };

    let range = builder.add(AnyType::Range(range_value.into()));

    builder.finish(AnyType::Seq(
        TSeq::try_new(range, 1, 20).unwrap(),
    ))
}

fn create_schema_supplied_inclusion() -> impl Schema<'static> {
    let mut builder = builder();

    let range_element = builder.add(AnyType::UInt(
        TUInt::try_new(5, 150).unwrap(),
    ));

    let range_value = TRange {
        meta : Meta::empty(),
        element: range_element,
        inclusion: Inclusion::Supplied,
        allow_empty: false,
    };

    let range = builder.add(AnyType::Range(range_value.into()));

    builder.finish(AnyType::Seq(
        TSeq::try_new(range, 1, 20).unwrap(),
    ))
}

#[test]
fn ok_1() {
    let schema = create_schema_given_inclusion();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_ok.yaml"),
    ))
}

#[test]
fn err_equal() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_err_equal.yaml"),
    ))
}

#[test]
fn err_start_end_ord() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_err_start_end_ord.yaml"),
    ))
}

#[test]
fn ok_2() {
    let schema = create_schema_supplied_inclusion();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_supplied_inclusion_ok.yaml"),
    ))
}
