//! Parsing tests for external EDS files
use seds_rs::{
    eds::{raw::PackageFile, resolve::Resolve},
    expr::ExpressionContext,
};
use serde_xml_rs::from_str;
use std::fs;
use walkdir::{DirEntry, WalkDir};

/// filter for determining eds
fn is_xml(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".xml")
}

/// Parse an XML string into a `PackageFile` struct
fn parse_xml<DeError: std::convert::From<serde_xml_rs::Error>>(
    xml: &str,
) -> Result<PackageFile, DeError> {
    let package_file: PackageFile = from_str(xml)?;
    Ok(package_file)
}

/// attempt to parse all the EDS XML files in a directory
fn test_directory(directory_name: &str) {
    // assert the directory exists
    assert!(
        std::path::Path::new(directory_name).exists(),
        "Directory does not exist: {}",
        directory_name
    );
    let walker = WalkDir::new(directory_name).into_iter();
    for entry in walker.filter_map(Result::ok).filter(is_xml) {
        println!("Parsing: {}", entry.path().display());
        let xml_string = fs::read_to_string(entry.path()).expect("Could not read the XML file");
        let res = parse_xml::<serde_xml_rs::Error>(&xml_string);
        assert!(
            res.is_ok(),
            "Failed to parse file: {}, Error: {:?}",
            entry.path().display(),
            res
        );
        let package = res.unwrap();
        let json = serde_json::json!({
            "CCSDS_SPACEPACKET": {
                "HEADER_TYPE": "<HEADER_TYPE>",
            },
            "CFE_MISSION": {
                "TELEMETRY_SUBSECONDS_TYPE": "<SUBSECONDS_TYPE>",
                "SIGNED_INTEGER_ENCODING": "signMagnitude",
                "DATA_BYTE_ORDER": "littleEndian",
                "MEM_REFERENCE_SIZE_BITS": "2",
                "ES_CDS_MAX_FULL_NAME_LEN": "2",
                "EVS_MAX_MESSAGE_LENGTH": "2",
                "MAX_CPU_ADDRESS_SIZE": "1024",
                "ES_MAX_APPLICATIONS": "2",
                "FS_HDR_DESC_MAX_LEN": "2",
                "MAX_PATH_LEN": "2",
                "MAX_API_LEN": "2",
                "SB_MAX_PIPES": "2",
                "ES_PERF_MAX_IDS": "2",
                "ES_POOL_MAX_BUCKETS": "2",
                "TBL_MAX_FULL_NAME_LEN": "2",
            },
            "CFE_SB": {
                "MSGID_BIT_SIZE": "2",
                "SUB_ENTRIES_PER_PKT": "2",
            },
            "CFE_FS": {
                "HDR_DESC_MAX_LEN": "2",
            },
        });
        let ectx = ExpressionContext::from_json(&json).unwrap();
        let res_ast = package.resolve(&ectx);
        assert!(
            res_ast.is_ok(),
            "Failed to resolve file: {}, Error: {:?}",
            entry.path().display(),
            res_ast
        );
    }
}

/// test parsing our basic EDS files
#[test]
fn test_ast_basic() {
    test_directory("eds/basic");
}

/// test parsing NASA's cFE EDS files
#[test]
fn test_ast_cfe() {
    test_directory("eds/cFE");
}

/// test parsing SANA EDS files
#[test]
fn test_ast_sana() {
    test_directory("eds/SEDSDoTForSANA");
}
