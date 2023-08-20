use common::{get_mission_params, open_file};
use seds_rs::codegen::context::{CodegenContext, Namespace};
use seds_rs::eds::ast::PackageFile;
use seds_rs::eds::raw;
use seds_rs::{
    codegen::{convert::ToRustMod, rustfmt},
    eds::resolve::Resolve,
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
    let pf = packagefiles[1].clone();
    let pfs: Vec<&PackageFile> = packagefiles.iter().collect();

    let pkg = pf.package[0].clone();
    let namespace = Namespace::try_from(pfs).unwrap();
    let locals = Namespace::try_from(&pkg).unwrap();

    let ctx = CodegenContext {
        name: None,
        locals: &locals,
        namespace: &namespace,
    };
    //let code = rustfmt(pf.to_rust_mod(&ctx).unwrap()).unwrap();

    let mut spacepacket = pf.to_rust_mod(&ctx).unwrap();

    let npkg = packagefiles[0].package[0].clone();
    let nctx = CodegenContext {
        name: None,
        locals: &Namespace::try_from(&npkg).unwrap(),
        namespace: &namespace,
    };
    spacepacket.extend(packagefiles[0].to_rust_mod(&nctx).unwrap());
    let code = rustfmt(spacepacket).unwrap();

    // write to a tmp file
    use std::fs::File;
    use std::io::Write;
    // make sure the directory exists and make it if not
    std::fs::create_dir_all("test_output").unwrap();
    let mut file = File::create("test_output/spacepacket_new.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
