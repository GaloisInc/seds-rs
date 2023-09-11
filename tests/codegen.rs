use common::{get_mission_params, open_file};
use seds_rs::codegen::codegen_packagefiles;
use seds_rs::eds::ast::PackageFile;
use seds_rs::eds::raw;
use seds_rs::{codegen::rustfmt, eds::resolve::Resolve};

mod common;

/// test codegen on multiple cFE eds files
#[test]
fn test_cfe_codegen() {
    let paths = vec![
        "eds/cFE/modules/core_api/eds/cfe_fs.xml",
        "eds/cFE/modules/core_api/eds/ccsds_spacepacket.xml",
        "eds/cFE/modules/core_api/eds/base_types.xml",
        "eds/cFE/modules/fs/eds/cfe_fs.xml",
        "eds/cFE/modules/resourceid/eds/cfe_resourceid.xml",
        // "eds/cFE/modules/sb/eds/cfe_sb.xml", // ArrayDataType is not yet supported
        // "eds/cFE/modules/evs/eds/cfe_evs.xml", // ArrayDataType is not yet supported
        // "eds/cFE/modules/tbl/eds/cfe_tbl.xml", // depends on CFE_HDR/CommandHeader
        // "eds/cFE/modules/time/eds/cfe_time.xml", // 1HzCmd is not a valid Ident
        // "eds/cFE/modules/cfe_testcase/eds/cfe_testcase.xml", // depends on CFE_HDR
        // "eds/cFE/modules/core_private/eds/base_types.xml", // InvalidBitSize(8192)
        // "eds/cFE/modules/es/eds/cfe_es.xml", // Unsupported ArrayDataType
    ];

    test_paths_codegen(&paths);
}

/// test the codegen on multiple example eds files
#[test]
fn test_example_codegen() {
    let paths = vec![
        "eds/test/test_eds.xml",
        "eds/test/simplified_spacepacket_bacn.xml",
        "eds/test/otc.xml",
        // "eds/test/test_package.xml", // we don't support nested packages (yet, easy to do)
        "eds/test/test_multi_datasheet.xml",
        "eds/test/test_datatypes.xml",
        "eds/test/test_resolved.xml",
        "eds/test/simplified_spacepacket.xml",
        // "eds/test/test_container.xml", // we don't support List's yet
        "eds/test/test_datasheet.xml",
    ];

    test_paths_codegen(&paths);
}

fn test_paths_codegen(paths: &[&str]) {
    let rpackagefiles: Vec<raw::PackageFile> = paths
        .iter()
        .map(|fp| serde_xml_rs::from_str(&open_file(fp)).unwrap())
        .collect();

    let ectx = get_mission_params();
    let packagefiles: Vec<PackageFile> = rpackagefiles
        .iter()
        .map(|rpf| rpf.resolve(&ectx).unwrap())
        .collect();
    let pfs: Vec<&PackageFile> = packagefiles.iter().collect();

    let code_tokens = codegen_packagefiles(&pfs).unwrap();
    let code = rustfmt(code_tokens).unwrap();

    // write to a tmp file
    use std::fs::File;
    use std::io::Write;
    // make sure the directory exists and make it if not
    std::fs::create_dir_all("test_output").unwrap();
    let mut file = File::create("test_output/cfe.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
