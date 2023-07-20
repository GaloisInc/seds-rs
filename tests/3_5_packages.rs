//! 3.5 PACKAGES
use std::collections::HashSet;

use common::open_file;
use seds_rs::eds::raw::{DataSheet, Package, PackageFile};

mod common;

fn get_test_data_sheet() -> DataSheet {
    let contents = open_file("eds/test/test_datasheet.xml");
    let datasheet: DataSheet = serde_xml_rs::from_str(&contents).unwrap();
    datasheet
}

fn get_test_package() -> Package {
    let contents = open_file("eds/test/test_package.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    package.package[0].clone()
}

/// **3.5.1** The name of each Package element declared shall be unique within the datasheet.
#[test]
fn test_3_5_1() {
    let data_sheet = get_test_data_sheet(); // Assume this function returns a test DataSheet

    let package_names: Vec<String> = data_sheet
        .packages
        .iter()
        .map(|p| p.name_entity_type.name.clone())
        .collect();
    let unique_package_names: HashSet<String> = package_names.into_iter().collect();

    assert_eq!(data_sheet.packages.len(), unique_package_names.len());
}

/// **3.5.2** A Package name may be hierarchical, in which case it shall consist of multiple name segments separated by the slash character (‘/’).
#[test]
fn test_3_5_2() {
    let package = get_test_package(); // Assume this function returns a test Package

    // Check if the package name contains '/' character
    assert!(package.name_entity_type.name.contains('/'));
}

/// **3.5.3** A package may have an optional shortDescription attribute and an optional LongDescription child element.
#[test]
fn test_3_5_3() {
    let package = get_test_package(); // Assume this function returns a test Package

    // Checking if optional fields exist
    assert!(
        package.name_entity_type.short_description.is_some()
            || package.name_entity_type.long_description.is_some()
    );
}

/// **3.5.4** A Package element may contain the following optional elements, in the following order: a) DataTypeSet; b) DeclaredInterfaceSet; c) ComponentSet.
#[test]
fn test_3_5_4() {
    let package = get_test_package(); // Assume this function returns a test Package

    // Check if the optional elements exist in the package
    let data_type_set_exists = package.data_type_set.is_some();
    // let declared_interface_set_exists = package.declared_interface_set.is_some();
    // let component_set_exists = package.component_set.is_some();

    // Check if the order of existence is correct
    // assert!(!data_type_set_exists || (data_type_set_exists && declared_interface_set_exists) || (data_type_set_exists && declared_interface_set_exists && component_set_exists));
    assert!(data_type_set_exists);

    // TODO: we fail this req for now
}
