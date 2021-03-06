use liquesco_schema::any_type::AnyType;

pub(crate) fn type_description(any_type: &AnyType) -> (u8, &'static str, &'static str) {
    match any_type {
            AnyType::Struct(_) => (0, "structure", "A structure (aka struct) contains 0-n fields. \
            The fields do not need to be of the same type."),
            AnyType::Binary(_) => (1, "binary", "Arbitrary binary data."),
            AnyType::UInt(_) => (2, "unsigned integer", "Data of an unsigned integer (aka uint or \
            unsigned int) holds a single (positive) integer value (within a defined range)."),
            AnyType::SInt(_) => (3, "signed integer", "Data of signed integer (aka sint or signed \
            int) holds a single (positive or negative) integer value (within a defined range)."),
            AnyType::Ascii(_) => (4, "ascii", "Ascii (aka ascii text) is a sequence of characters \
            where each of them is within the ascii range (0-127 inclusive). It can be used to \
            transfer technical text (aka string); it's not to be used to transfer human readable \
            text (use unicode for this case)."),
            AnyType::Bool(_) => (5, "boolean", "Data of type boolean (aka bool) can hold the value \
            'true' or the value 'false' (1 or 0; on or off; enabled or disabled). It's like a \
            single bit of information. As an alternative you an also use an enum with 2 variants \
            (it's usually better suited in most cases)"),
            AnyType::Enum(_) => (6, "enumeration", "An enumeration (aka enum; tagged union; variant \
            record; discriminated union) contains 1-n variants; Each variant can (optionally) \
            have a value."),
            AnyType::Seq(_) => (7, "sequence", "A sequence (aka seq; list; vector; array) describes \
            a sequence of 0-n elements. Unlike struct fields, each element in a sequence has to \
            be of the same type."),
            AnyType::Float32(_) => (8, "float32", "A IEEE 754 32 bit float number. Do not use this \
            to transfer decimal values."),
            AnyType::Float64(_) => (9, "float64", "A IEEE 754 64 bit float number. Do not use this \
            to transfer decimal values."),
            AnyType::Option(_) => (10, "option", "Use the option type (aka maybe; optional; nullable) \
            to describe data that can either be there ('present'; 'some') or absent ('missing'; \
            'empty'). Alternatively you can also use an enum with two variants to achieve the same."),
            AnyType::Unicode(_) => (11, "unicode", "The unicode type (aka string) can be used to \
            describe arbitrary human readable text."),
            AnyType::Uuid(_) => (12, "uuid", "16 byte UUID; RFC 4122."),
            AnyType::Range(_) => (13, "range", "A range (start - end); start/end with configurable \
            inclusion/exclusion."),
            AnyType::Map(_) => (14, "map", "A sequence of key-value entries. Duplicate keys are not \
            allowed. The keys can optionally be referenced to create recursive data structures."),
            AnyType::RootMap(_) => (15, "root_map", "A map with a root. Keys have to be unique. \
            The keys can be referenced. The root cannot be referenced. The root can reference keys."),
            AnyType::KeyRef(_) => (16, "key_ref", "Key references can reference keys from outer types \
            that supports references (provide anchors that can be referenced): Maps and RootMaps."),
            AnyType::Decimal(_) => (17, "decimal", "A normalized decimal number. It's composed of \
            a signed 128 bit coefficient (at max) and a signed 8 bit exponent (at max) (c*10^e)."),
    }
}
