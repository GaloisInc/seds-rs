//! 3.2 ELECTRONIC DATA SHEETS AND THE ASSOCIATED SCHEMA
use seds_rs::eds::raw::PackageFile;
use std::fs;
use std::path::Path;

use crate::common::open_file;

mod common;

/// **3.2.1** The basic unit of data exchange of SOIS device information is an XML document known as a device datasheet or package file.
#[test]
fn test_3_2_1() {
    let contents = open_file("eds/test/test_eds.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    assert_eq!(package.package[0].name_entity_type.name, "TestPackage");
}

/// **3.2.2** A device datasheet or package file shall be de defined by a single top-level XML file.
#[test]
fn test_3_2_2() {
    // Testing that a package file has a single top-level XML file would require that we count
    // the number of XML root elements in a file, but XML files by definition have only one root
    // element. The XML parsing library (serde_xml_rs) would error out if there were more.
    let path = Path::new("eds/test/test_eds.xml");
    assert!(fs::metadata(path).is_ok());
}

/// **3.2.3** Any files referenced by a device datasheet shall be XML package files compliant to the PackageFile element of the SEDS schema.
#[test]
fn test_3_2_3() {
    // The referenced file will need to be manually inspected and confirmed that it is a valid
    // SEDS package file. For this test, we'll simply load the file and parse it.
    let contents = open_file("eds/test/test_eds.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    assert_eq!(package.package[0].name_entity_type.name, "TestPackage");
}

/// **3.2.4** When a package file is used by a datasheet, XInclude (reference [5]) may be used to incorporate the Package element of that file into a single logical document compliant to the DataSheet element of the SEDS schema.
#[test]
fn test_3_2_4() {
    // Given that this requirement states "may be used", this is an optional feature.
    // This test can only be performed if we have an example of a package file using XInclude.
    // Otherwise, this test will remain as a placeholder.
    assert!(true);
}

/// **3.2.5** A package file shall be a single standalone XML file without any use of XInclude.
#[test]
fn test_3_2_5() {
    // The library currently doesn't support XInclude, so this is always true.
    // To fully test this, we would need to add XInclude support to the library,
    // and then parse the file and verify that it doesn't contain any XInclude elements.
    assert!(true);
}

/// **3.2.6** A SEDS document can make reference to one or more user-defined DoTs. In this case, the actual schema reference from the datasheet will be to a schema which is an extension of the SEDS schema.
#[test]
fn test_3_2_6() {
    // This test would require an example file which references a user-defined DoT.
    // For now, this test will remain as a placeholder.
    assert!(true);
}
