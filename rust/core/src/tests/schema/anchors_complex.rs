use crate::schema::anchors::TAnchors;
use crate::schema::ascii::TAscii;
use crate::schema::core::Schema;
use crate::schema::reference::TReference;
use crate::schema::seq::TSeq;
use crate::schema::structure::TStruct;
use crate::tests::schema::builder::builder;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::id;

use serde::{Deserialize, Serialize};

#[test]
fn can_reference_multiple() {
    let schema = create_schema();
    assert_valid_strict(
        (
            Complex {
                text: "hello master!".to_string(),
                children: vec![1, 2, 3],
            },
            vec![
                Complex {
                    text: "index 1".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 2".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 3".to_string(),
                    children: vec![],
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn can_reference_multiple_overflow() {
    let schema = create_schema();
    assert_invalid_strict(
        (
            Complex {
                text: "hello master!".to_string(),
                // note: index 4 does not exist
                children: vec![1, 2, 3, 4],
            },
            vec![
                Complex {
                    text: "index 1".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 2".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 3".to_string(),
                    children: vec![],
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn unused_item() {
    let schema = create_schema();
    assert_invalid_strict(
        (
            Complex {
                text: "hello master!".to_string(),
                children: vec![1, 2, 3],
            },
            vec![
                Complex {
                    text: "index 1".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 2".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 3".to_string(),
                    children: vec![],
                },
                // this is not referenced
                Complex {
                    text: "index 4".to_string(),
                    children: vec![],
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn five_anchors() {
    let schema = create_schema();
    assert_valid_strict(
        (
            Complex {
                text: "hello master!".to_string(),
                children: vec![1, 2, 3],
            },
            vec![
                Complex {
                    text: "index 1".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 2".to_string(),
                    children: vec![],
                },
                Complex {
                    text: "index 3".to_string(),
                    children: vec![4],
                },
                // referenced by 3, references 0 again.
                Complex {
                    text: "index 4".to_string(),
                    children: vec![0],
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn can_back_reference() {
    let schema = create_schema();
    assert_valid_strict(
        (
            Complex {
                text: "hello master!".to_string(),
                children: vec![1, 2, 3],
            },
            vec![
                Complex {
                    text: "index 1".to_string(),
                    children: vec![3, 2, 1],
                },
                Complex {
                    text: "index 2".to_string(),
                    children: vec![2, 3, 1],
                },
                Complex {
                    text: "index 3".to_string(),
                    children: vec![4, 1, 3, 2],
                },
                Complex {
                    text: "index 4".to_string(),
                    children: vec![0, 1, 2, 3, 4],
                },
            ],
        ),
        &schema,
    );
}

fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();
    let reference = builder.add(TReference);
    let text = builder.add(TAscii::try_new(0, 100, 0, 127).unwrap());
    let children = builder.add(TSeq::try_new(reference, 0, 1000).unwrap());
    let structure = builder.add(
        TStruct::builder()
            .field(id("text"), text)
            .field(id("children"), children)
            .build(),
    );
    builder.finish(TAnchors::new(structure, structure))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Complex {
    text: String,
    children: Vec<u32>,
}
