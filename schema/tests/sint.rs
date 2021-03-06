mod common;

use common::ordering::ord_assert_ascending;
use common::ordering::ord_assert_equal;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use common::utils::single_schema;
use liquesco_schema::types::sint::TSInt;

#[test]
fn schema1() {
    let schema = single_schema(TSInt::try_new(-45, 4443444).unwrap());

    // some valid items
    assert_valid_strict(-45isize, &schema);
    assert_valid_strict(5isize, &schema);
    assert_valid_strict(4_443_444isize, &schema);
    assert_valid_strict(4_443_443isize, &schema);
    assert_valid_strict(0isize, &schema);

    // some invalid items
    assert_invalid_strict(-46isize, &schema);
    assert_invalid_strict(4_443_445isize, &schema);
    assert_invalid_strict(std::i64::MIN, &schema);
    assert_invalid_strict(std::i64::MAX, &schema);
}

#[test]
fn schema_big() {
    let schema = single_schema(TSInt::try_new(std::i128::MIN + 2, std::i128::MAX - 2).unwrap());

    // some valid items
    assert_valid_strict(std::i128::MIN + 2, &schema);
    assert_valid_strict(std::i128::MAX - 2, &schema);

    // some invalid items
    assert_invalid_strict(std::i128::MIN + 1, &schema);
    assert_invalid_strict(std::i128::MIN, &schema);
    assert_invalid_strict(std::i128::MAX - 1, &schema);
    assert_invalid_strict(std::i128::MAX, &schema);
}

#[test]
fn ordering() {
    let schema = TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap();

    ord_assert_equal(schema.clone(), -100isize, -100isize);
    ord_assert_equal(schema.clone(), 0isize, 0isize);
    ord_assert_equal(schema.clone(), std::i64::MAX, std::i64::MAX);

    ord_assert_ascending(schema.clone(), 0i64, std::i64::MAX);
    ord_assert_ascending(schema.clone(), -5isize, 6isize);
}
