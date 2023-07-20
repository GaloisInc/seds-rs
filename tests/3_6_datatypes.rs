use std::collections::HashSet;

use seds_rs::eds::raw::{DataType, Package, PackageFile, DataTypeSet};

mod common;

use common::open_file;

fn get_test_package() -> Package {
    let contents = open_file("eds/test/test_datatypes.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    package.package[0].clone()
}

fn get_test_data_type_set() -> DataTypeSet {
    let contents = open_file("eds/test/test_datatypes.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    package.package[0].clone().data_type_set.unwrap()
}

/// 3.6.1 The DataTypeSet element contained in a package or component shall contain one or more of the following elements: 
/// ArrayDataType, BinaryDataType, BooleanDataType, ContainerDataType, EnumeratedDataType, FloatDataType, IntegerDataType, StringDataType, and SubRangeDataType.
#[test]
fn test_3_6_1() {
    let package = get_test_package(); // Assume this function returns a test Package

    assert!(!package.data_type_set.unwrap().data_types.is_empty());
}

/// 3.6.2 Each child element of a DataTypeSet element is based on the NamedEntityType (see 3.3.7).
#[test]
fn test_3_6_2() {
    let data_type_set = get_test_data_type_set(); // Assume this function returns a test DataTypeSet

    // Check for each DataType in DataTypeSet if it's based on NamedEntityType
    for data_type in data_type_set.data_types {
        match data_type {
            DataType::ArrayDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::BooleanDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::ContainerDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::EnumeratedDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::FloatDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::IntegerDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::StringDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            DataType::SubRangeDataType(data) => assert!(!data.name_entity_type.name.is_empty()),
            _ => (),
        }
    }
}

/// 3.6.3 The name of each child element of a DataTypeSet element shall be unique within the containing package.
#[test]
fn test_3_6_3() {
    let data_type_set = get_test_data_type_set(); // Assume this function returns a test DataTypeSet

    let data_type_names: Vec<String> = data_type_set.data_types.iter()
        .map(|dt| {
            match dt {
                DataType::ArrayDataType(data) => data.name_entity_type.name.clone(),
                DataType::BooleanDataType(data) => data.name_entity_type.name.clone(),
                DataType::ContainerDataType(data) => data.name_entity_type.name.clone(),
                DataType::EnumeratedDataType(data) => data.name_entity_type.name.clone(),
                DataType::FloatDataType(data) => data.name_entity_type.name.clone(),
                DataType::IntegerDataType(data) => data.name_entity_type.name.clone(),
                DataType::StringDataType(data) => data.name_entity_type.name.clone(),
                DataType::SubRangeDataType(data) => data.name_entity_type.name.clone(),
                _ => String::new(),
            }
        })
        .collect();
    let unique_data_type_names: HashSet<String> = data_type_names.into_iter().collect();

    assert_eq!(data_type_set.data_types.len(), unique_data_type_names.len());
}
