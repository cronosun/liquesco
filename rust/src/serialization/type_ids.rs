use crate::serialization::core::MajorType;

// Minimum type ID is 0, maximum is 24 (inclusive).

pub const TYPE_BOOL_FALSE: MajorType = MajorType::new(0);
pub const TYPE_BOOL_TRUE: MajorType = MajorType::new(1);
pub const TYPE_OPTION: MajorType = MajorType::new(2);
pub const TYPE_LIST: MajorType = MajorType::new(3);
pub const TYPE_BINARY: MajorType = MajorType::new(4);
pub const TYPE_UTF8: MajorType = MajorType::new(5);

pub const TYPE_ENUM_0: MajorType = MajorType::new(6);
pub const TYPE_ENUM_1: MajorType = MajorType::new(7);
pub const TYPE_ENUM_2: MajorType = MajorType::new(8);
pub const TYPE_ENUM_N: MajorType = MajorType::new(9);

pub const TYPE_UINT: MajorType = MajorType::new(10);
pub const TYPE_SINT: MajorType = MajorType::new(11);

pub const TYPE_UUID: MajorType = MajorType::new(12);

//pub const TYPE_DEC128: TypeId = TypeId::new(12);

// -> ne, glaube custom brauchts nicht.. dazu haben wir ja das schema
// custom0: 20
// custom1: 21
// custom2: 22
// custom3: 23
// custom_arb: 24

// TODO: Integer types (2), timestamp (1), time of day, day (calendar), floats (2), Decimal128, Extension, UUID? reverse domain?
