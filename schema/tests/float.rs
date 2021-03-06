mod common;

use common::ordering::ord_assert_ascending;
use common::ordering::ord_assert_equal;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use common::utils::single_schema;
use liquesco_common::float::F32Ext;
use liquesco_common::float::F64Ext;
use liquesco_common::range::NewFull;
use liquesco_common::range::Range;
use liquesco_schema::types::float::TFloat32;
use liquesco_schema::types::float::TFloat64;

#[test]
fn schema1_32() {
    let schema = single_schema(TFloat32::new(Range::<F32Ext>::full()));

    // some valid items
    assert_valid_strict(-458.0f32, &schema);
    assert_valid_strict(458.0f32, &schema);
    assert_valid_strict(std::f32::MIN, &schema);
    assert_valid_strict(std::f32::MAX, &schema);
    assert_valid_strict(std::f32::MIN_POSITIVE, &schema);

    // some invalid items
    assert_invalid_strict(std::f32::NAN, &schema);
    assert_invalid_strict(std::f32::NEG_INFINITY, &schema);
    assert_invalid_strict(std::f32::INFINITY, &schema);
    // err: subnormal
    assert_invalid_strict(1.0e-40_f32, &schema);
    assert_invalid_strict(-1.0e-40_f32, &schema);
    // err: zero
    assert_invalid_strict(0.0f32, &schema);
    assert_invalid_strict(-0.0f32, &schema);
}

#[test]
fn schema1_64() {
    let schema = single_schema(TFloat64::new(Range::<F64Ext>::full()));

    // some valid items
    assert_valid_strict(-458.0f64, &schema);
    assert_valid_strict(458.0f64, &schema);
    assert_valid_strict(std::f64::MIN, &schema);
    assert_valid_strict(std::f64::MAX, &schema);
    assert_valid_strict(std::f64::MIN_POSITIVE, &schema);

    // some invalid items
    assert_invalid_strict(std::f64::NAN, &schema);
    assert_invalid_strict(std::f64::NEG_INFINITY, &schema);
    assert_invalid_strict(std::f64::INFINITY, &schema);
    // err: subnormal
    assert_invalid_strict(1.0e-308_f64, &schema);
    assert_invalid_strict(-1.0e-308_f64, &schema);
    // err: zero
    assert_invalid_strict(0.0f64, &schema);
    assert_invalid_strict(-0.0f64, &schema);
}

#[test]
fn schema2_32() {
    let float = TFloat32::new(
        Range::<F32Ext>::try_new_inclusive(F32Ext::from(-14.5f32), F32Ext::from(19.7f32)).unwrap(),
    )
    .with_allow_nan(true)
    .with_allow_positive_infinity(true)
    .with_allow_negative_infinity(true)
    .with_allow_positive_zero(true)
    .with_allow_negative_zero(true)
    .with_allow_subnormal(true);

    let schema = single_schema(float);

    // some valid items
    assert_valid_strict(-14.5f32, &schema);
    assert_valid_strict(19.7f32, &schema);
    assert_valid_strict(-14.49f32, &schema);
    assert_valid_strict(19.69f32, &schema);
    assert_valid_strict(std::f32::NAN, &schema);
    assert_valid_strict(std::f32::NEG_INFINITY, &schema);
    assert_valid_strict(std::f32::INFINITY, &schema);
    assert_valid_strict(0.0f32, &schema);
    assert_valid_strict(-0.0f32, &schema);
    assert_valid_strict(1.0e-40_f32, &schema);
    assert_valid_strict(-1.0e-40_f32, &schema);

    // some invalid items
    assert_invalid_strict(-14.51f32, &schema);
    assert_invalid_strict(19.71f32, &schema);
}

#[test]
fn schema2_64() {
    let float = TFloat64::new(
        Range::<F64Ext>::try_new_inclusive(F64Ext::from(-14.5f64), F64Ext::from(19.7f64)).unwrap(),
    )
    .with_allow_nan(true)
    .with_allow_positive_infinity(true)
    .with_allow_negative_infinity(true)
    .with_allow_positive_zero(true)
    .with_allow_negative_zero(true)
    .with_allow_subnormal(true);

    let schema = single_schema(float);

    // some valid items
    assert_valid_strict(-14.5f64, &schema);
    assert_valid_strict(19.7f64, &schema);
    assert_valid_strict(-14.49f64, &schema);
    assert_valid_strict(19.69f64, &schema);
    assert_valid_strict(std::f64::NAN, &schema);
    assert_valid_strict(std::f64::NEG_INFINITY, &schema);
    assert_valid_strict(std::f64::INFINITY, &schema);
    assert_valid_strict(0.0f64, &schema);
    assert_valid_strict(-0.0f64, &schema);
    assert_valid_strict(1.0e-308_f64, &schema);
    assert_valid_strict(-1.0e-308_f64, &schema);

    // some invalid items
    assert_invalid_strict(-14.51f64, &schema);
    assert_invalid_strict(19.71f64, &schema);
}

#[test]
fn ordering_64() {
    let schema = TFloat64::new(
        Range::<F64Ext>::try_new_inclusive(std::f64::MIN.into(), std::f64::MAX.into()).unwrap(),
    )
    .with_allow_nan(true)
    .with_allow_positive_infinity(true)
    .with_allow_negative_infinity(true);

    // nan is equal to itself
    ord_assert_equal(schema.clone(), std::f64::NAN, std::f64::NAN);
    // infinity is equal to itself
    ord_assert_equal(schema.clone(), std::f64::INFINITY, std::f64::INFINITY);
    ord_assert_equal(
        schema.clone(),
        std::f64::NEG_INFINITY,
        std::f64::NEG_INFINITY,
    );
    // and values of course
    ord_assert_equal(schema.clone(), 1.278f64, 1.278f64);

    // nan is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f64::NAN, -100f64);
    ord_assert_ascending(schema.clone(), std::f64::NAN, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::NAN, std::f64::NEG_INFINITY);

    // except for nan, negative infinity is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, -100f64);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::MIN);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::INFINITY);

    // positive infinity is always the largest thing
    ord_assert_ascending(schema.clone(), 1000000f64, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::MAX, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::INFINITY);

    // and normal values
    ord_assert_ascending(schema.clone(), 0.01f64, 0.011f64);
}

#[test]
fn ordering_32() {
    let schema = TFloat32::new(
        Range::<F32Ext>::try_new_inclusive(std::f32::MIN.into(), std::f32::MAX.into()).unwrap(),
    )
    .with_allow_nan(true)
    .with_allow_positive_infinity(true)
    .with_allow_negative_infinity(true);

    // nan is equal to itself
    ord_assert_equal(schema.clone(), std::f32::NAN, std::f32::NAN);
    // infinity is equal to itself
    ord_assert_equal(schema.clone(), std::f32::INFINITY, std::f32::INFINITY);
    ord_assert_equal(
        schema.clone(),
        std::f32::NEG_INFINITY,
        std::f32::NEG_INFINITY,
    );
    // and values of course
    ord_assert_equal(schema.clone(), 1.278f32, 1.278f32);

    // nan is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f32::NAN, -100f32);
    ord_assert_ascending(schema.clone(), std::f32::NAN, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::NAN, std::f32::NEG_INFINITY);

    // except for nan, negative infinity is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, -100f32);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::MIN);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::INFINITY);

    // positive infinity is always the largest thing
    ord_assert_ascending(schema.clone(), 1_000_000f32, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::MAX, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::INFINITY);

    // and normal values
    ord_assert_ascending(schema.clone(), 0.01f32, 0.011f32);
}
