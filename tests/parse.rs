use seds_rs::package::PackageFile;
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
    }
}

#[test]
fn test_parsing_basic() {
    test_directory("eds/basic");
}

#[test]
fn test_parsing_cfe() {
    test_directory("eds/cFE");
}
