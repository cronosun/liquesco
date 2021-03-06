mod common;

use common::builder::builder;
use common::builder::into_schema;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::core::Schema;
use liquesco_schema::types::seq::TSeq;
use std::convert::TryFrom;

use liquesco_schema::identifier::Identifier;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::types::key_ref::TKeyRef;
use liquesco_schema::types::map::TMap;
use liquesco_schema::types::structure::Field;
use liquesco_schema::types::structure::TStruct;
use liquesco_schema::types::unicode::LengthType;
use liquesco_schema::types::unicode::TUnicode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[test]
fn ok_empty() {
    let schema = create_schema1();
    let map: BTreeMap<String, Value> = BTreeMap::new();
    assert_valid_strict(map, &schema);
}

#[test]
fn ok_no_references() {
    let schema = create_schema1();
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert(
        "item_a".to_string(),
        Value {
            text: "Some text".to_string(),
            refs: vec![],
        },
    );
    map.insert(
        "item_b".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![],
        },
    );
    assert_valid_strict(map, &schema);
}

#[test]
fn ok_with_references() {
    let schema = create_schema1();
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert(
        "item_a".to_string(),
        Value {
            text: "Some text".to_string(),
            refs: vec![0, 0, 0, 2],
        },
    );
    map.insert(
        "item_b".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![2, 2, 2],
        },
    );
    map.insert(
        "item_c".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![1],
        },
    );
    assert_valid_strict(map, &schema);
}

#[test]
fn err_out_of_index() {
    let schema = create_schema1();
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert(
        "item_a".to_string(),
        Value {
            text: "Some text".to_string(),
            refs: vec![0, 0, 0, 2],
        },
    );
    map.insert(
        "item_b".to_string(),
        Value {
            text: "Some other text".to_string(),
            // this "3" is ouf of index
            refs: vec![2, 2, 2, 3],
        },
    );
    map.insert(
        "item_c".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![1],
        },
    );
    assert_invalid_strict(map, &schema);
}

#[test]
fn err_map_provides_no_anchors() {
    let schema = create_schema_no_anchors();
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert(
        "item_a".to_string(),
        Value {
            text: "Some text".to_string(),
            refs: vec![0, 0, 0, 2],
        },
    );
    map.insert(
        "item_b".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![2, 2, 2],
        },
    );
    map.insert(
        "item_c".to_string(),
        Value {
            text: "Some other text".to_string(),
            refs: vec![1],
        },
    );
    assert_invalid_strict(map, &schema);
}

fn create_schema1() -> impl Schema {
    let mut builder = builder();
    let key = builder.add_unwrap(
        "key",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );

    let field_text = builder.add_unwrap(
        "unicode",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );
    let single_ref = builder.add_unwrap("key_ref", TKeyRef::default());
    let field_refs = builder.add_unwrap("key_ref_seq", TSeq::try_new(single_ref, 0, 100).unwrap());
    let value = builder.add_unwrap(
        "struct",
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("text").unwrap(),
                field_text,
            ))
            .add(Field::new(
                Identifier::try_from("refs").unwrap(),
                field_refs,
            )),
    );

    let root = builder.add_unwrap("root", TMap::new(key, value).with_anchors(true));
    into_schema(builder, root)
}

fn create_schema_no_anchors() -> impl Schema {
    let mut builder = builder();
    let key = builder.add_unwrap(
        "key",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );

    let field_text = builder.add_unwrap(
        "unicode",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );
    let single_ref = builder.add_unwrap("key_ref", TKeyRef::default());
    let field_refs = builder.add_unwrap("key_ref_seq", TSeq::try_new(single_ref, 0, 100).unwrap());
    let value = builder.add_unwrap(
        "struct",
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("text").unwrap(),
                field_text,
            ))
            .add(Field::new(
                Identifier::try_from("refs").unwrap(),
                field_refs,
            )),
    );

    // note: anchors is set to 'false'
    let root = builder.add_unwrap("root", TMap::new(key, value).with_anchors(false));
    into_schema(builder, root)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Value {
    text: String,
    refs: Vec<u32>,
}
