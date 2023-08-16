use common::{get_mission_params, open_file};
use seds_rs::eds::raw;
use seds_rs::{
    codegen::{rustfmt, ToRustMod},
    eds::{
        ast::{
            ByteOrder, Identifier, IntegerDataEncoding, IntegerDataType, IntegerEncoding,
            MinMaxRange, NamedEntityType, Range,
        },
        resolve::Resolve,
    },
};

mod common;

#[test]
fn test_integer_field() {
    let _ = IntegerDataType {
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

    //println!("{}", dt.to_deku_field(None).unwrap());
    //println!("{}", rustfmt(dt.to_deku_struct(None).unwrap()).unwrap());
}

#[test]
fn test_spacepacket() {
    let contents = open_file("eds/test/simplified_spacepacket.xml");
    let rpf: raw::PackageFile = serde_xml_rs::from_str(&contents).unwrap();

    let ectx = get_mission_params();
    let pf = rpf.resolve(&ectx).unwrap();

    let code = rustfmt(pf.to_rust_mod(None).unwrap()).unwrap();
    println!("{}", code);

    // write to a tmp file
    use std::fs::File;
    use std::io::Write;
    // make sure the directory exists and make it if not
    std::fs::create_dir_all("test_output").unwrap();
    let mut file = File::create("test_output/spacepacket.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

/// test against the supported files in the cFE
#[test]
fn test_cfe() {
    let contents = open_file("eds/cFE/modules/core_api/eds/base_types.xml");
    let rpf: raw::PackageFile = serde_xml_rs::from_str(&contents).unwrap();

    let ectx = get_mission_params();
    let pf = rpf.resolve(&ectx).unwrap();

    let code = rustfmt(pf.to_rust_mod(None).unwrap()).unwrap();
    println!("{}", code);

    // write to a tmp file
    use std::fs::File;
    use std::io::Write;
    // make sure the directory exists and make it if not
    std::fs::create_dir_all("test_output").unwrap();
    let mut file = File::create("test_output/spacepacket.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
