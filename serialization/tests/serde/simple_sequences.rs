use crate::serde::assert_serde;
use crate::serde::serialize_to_same;
use serde::{Deserialize, Serialize};

#[test]
fn empty_struct() {
    let empty_struct = EmptyStruct();
    assert_serde(empty_struct);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct EmptyStruct();

#[test]
fn one_field_struct() {
    let value = OneFieldStruct { integer: 45584 };
    assert_serde(value);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct OneFieldStruct {
    integer: usize,
}

#[test]
fn two_fields_struct() {
    let value = TwoFieldsStruct {
        x: 45584.5,
        y: 4.487,
    };
    assert_serde(value);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TwoFieldsStruct {
    x: f64,
    y: f64,
}

#[test]
fn three_fields_struct() {
    let value = ThreeFieldsStruct {
        first_name: "Albert".to_string(),
        last_name: "Einstein".to_string(),
        age: 141,
    };
    assert_serde(value);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ThreeFieldsStruct {
    first_name: String,
    last_name: String,
    age: usize,
}

#[test]
fn tuples() {
    assert_serde(());
    let single_tuple: (String) = "Hello".to_string();
    assert_serde(single_tuple);
    assert_serde(("Hello".to_string(), "World".to_string()));
    assert_serde(("Hello".to_string(), "World".to_string(), 345));
    assert_serde(("Hello".to_string(), "World".to_string(), 345, false));
    assert_serde((
        "Hello".to_string(),
        "World".to_string(),
        345,
        false,
        "end".to_string(),
    ));
    assert_serde((
        "Hello".to_string(),
        "World".to_string(),
        345,
        false,
        "end".to_string(),
        (),
    ));
}

#[test]
fn bytes_slice() {
    let slice: &'static [u8] = &[4, 0, 45, 0, 0, 0, 4, 78, 254, 255, 0];
    assert_serde(slice.to_vec());
}

#[test]
fn tuple_and_struct_serialize_to_same1() {
    serialize_to_same(TwoFieldsStruct { x: 48.447, y: 47.3 }, (48.447f64, 47.3f64));
}

#[test]
fn tuple_and_struct_serialize_to_same2() {
    serialize_to_same(
        ThreeFieldsStruct {
            first_name: "Maximus".to_string(),
            last_name: "Muster".to_string(),
            age: 45878,
        },
        ("Maximus".to_string(), "Muster".to_string(), 45878usize),
    );
}

#[test]
fn vec_and_struct_serialize_to_same() {
    serialize_to_same(
        TwoFieldsStruct {
            x: 4587.0,
            y: 47.244_774_7,
        },
        vec![4587.0f64, 47.244_774_7f64],
    );
}

#[test]
fn vec_and_tuple_serialize_to_same() {
    serialize_to_same(
        (
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "text".to_string(),
        ),
        vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "text".to_string(),
        ],
    );
}
