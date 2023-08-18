use common::{get_mission_params, open_file};
use seds_rs::codegen::RustTypeRefs;
use seds_rs::codegen::context::{Namespace, CodegenContext};
use seds_rs::eds::ast::PackageFile;
use seds_rs::eds::raw;
use seds_rs::{
    codegen::{rustfmt, convert_rust::ToRustMod},
    eds::{
        ast::{
            ByteOrder, Identifier, IntegerDataEncoding, IntegerDataType, IntegerEncoding,
            MinMaxRange, NamedEntityType, Range,
        },
        resolve::Resolve,
    },
};

mod common;

/// test the namespace on multiple eds files
#[test]
fn test_cfe_namespace() {
    let paths = vec![
        "eds/cFE/modules/core_api/eds/base_types.xml",
        "eds/cFE/modules/core_api/eds/ccsds_spacepacket.xml",
    ];
    let rpackagefiles: Vec<raw::PackageFile> = paths
        .iter()
        .map(|fp| serde_xml_rs::from_str(&open_file(fp)).unwrap())
        .collect();

    let ectx = get_mission_params();
    let packagefiles: Vec<PackageFile> = rpackagefiles
        .iter()
        .map(|rpf| rpf.resolve(&ectx).unwrap())
        .collect();
    let pf = packagefiles[0].clone();
    let pfs: Vec<&PackageFile> = packagefiles.iter().collect();

    let namespace = Namespace::from(pfs);

    println!("{:?}", namespace.find_type_item("BASE_TYPES/uint32"));

    let ctx = CodegenContext {
        name: None,
        type_refs: &RustTypeRefs{
            type_refs: todo!() 
        },
        namespace: &namespace
    };
    
    let code = rustfmt(pf.to_rust_mod(&ctx).unwrap()).unwrap();
    println!("{}", code);

    // write to a tmp file
    use std::fs::File;
    use std::io::Write;
    // make sure the directory exists and make it if not
    std::fs::create_dir_all("test_output").unwrap();
    let mut file = File::create("test_output/spacepacket_new.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
