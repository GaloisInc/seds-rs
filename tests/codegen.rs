use proc_macro2::Literal;
use seds_rs::{
    codegen::{rustfmt, ToDekuField, ToDekuStruct},
    eds::ast::{
        ByteOrder, Identifier, IntegerDataEncoding, IntegerDataType, IntegerEncoding, MinMaxRange,
        NamedEntityType, Range,
    },
};

#[test]
fn test_integer_field() {
    let dt = IntegerDataType {
        name_entity_type: NamedEntityType {
            name: Identifier("test".to_string()),
            short_description: Some("a test field".to_string()),
            long_description: None,
        },
        encoding: IntegerDataEncoding {
            size_in_bits: 8,
            encoding: IntegerEncoding::SignMagnitude,
            byte_order: ByteOrder::BigEndian,
        },
        range: Range {
            min_max_range: MinMaxRange {
                max: seds_rs::eds::ast::Literal("255".to_string()),
                min: seds_rs::eds::ast::Literal("0".to_string()),
                range_type: seds_rs::eds::ast::MinMaxRangeType::InclusiveMinInclusiveMax,
            },
        },
    };

    println!("{}", dt.to_deku_field().unwrap());
    println!("{}", rustfmt(dt.to_deku_struct().unwrap()).unwrap());
}
