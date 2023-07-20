//! 3.3 SEDS/XML BASIC STRUCTURE
use common::open_file;
use seds_rs::eds::raw::{DataSheet, PackageFile};

mod common;

/// **3.3.1** The root element of a SEDS document shall be one of the DataSheet and PackageFile elements.
#[test]
fn test_3_3_1() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let _: DataSheet = serde_xml_rs::from_str(&contents).unwrap();

    let contents = open_file("eds/test/test_package.xml");
    let _: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
}

/// **3.3.2** The DataSheet element shall contain exactly one Device element.
#[test]
fn test_3_3_2() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();
    assert_eq!(datasheet.devices.len(), 1);
}

/// **3.3.3** The DataSheet element shall contain one or more Package elements.
#[test]
fn test_3_3_3() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();
    assert!(datasheet.packages.len() >= 1);
}

/// **3.3.4** The PackageFile element shall contain exactly one Package element.
#[test]
fn test_3_3_4() {
    let contents = open_file("eds/test/test_package.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    assert_eq!(package.package.len(), 1);
}

/// **3.3.5** The Device and PackageFile elements shall contain zero or one MetaData elements (see 3.4).
#[test]
fn test_3_3_5() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();
    assert!(datasheet.devices[0].metadata.is_some());

    let contents = open_file("eds/test/test_package.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    assert!(package.package[0].metadata.is_some());
}

/// **3.3.6** If any SEDS element is based on the NamedEntityType, the element shall have a name attribute and may have the optional shortDescription attribute and LongDescription child element. Optionally, such an element may carry attributes specified by the standard DoT (reference [1]).
#[test]
fn test_3_3_6() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();

    for device in datasheet.devices {
        assert!(!device.name_entity_type.name.is_empty());
    }
}

/// **3.3.7** The Device element shall be based on the NamedEntityType.
#[test]
fn test_3_3_7() {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();

    for device in datasheet.devices {
        assert!(!device.name_entity_type.name.is_empty());
    }
}
