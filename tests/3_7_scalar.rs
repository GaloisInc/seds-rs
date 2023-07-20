use seds_rs::eds::raw::{DataType, DataTypeSet, PackageFile, Package};

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


/// 3.7.1 Each BooleanDataType, EnumeratedDataType, FloatDataType, IntegerDataType,
/// StringDataType, or SubRangeDataType element may contain an optional encoding element
/// of a type corresponding to table 3-1.
#[test]
fn test_3_7_1() {
    let data_type_set = get_test_data_type_set(); // Assume this function returns a test DataTypeSet

    for data_type in data_type_set.data_types {
        match data_type {
            DataType::BooleanDataType(data) => assert!(data.encoding.is_some()),
            DataType::EnumeratedDataType(data) => assert!(data.encoding.is_some()),
            DataType::FloatDataType(data) => assert!(data.encoding.is_some()),
            DataType::IntegerDataType(data) => assert!(data.encoding.is_some()),
            DataType::StringDataType(data) => assert!(data.encoding.is_some()),
            DataType::SubRangeDataType(data) => assert!(data.encoding.is_some()),
            _ => (),
        }
    }
}

/// 3.7.2 A FloatDataEncoding or IntegerDataEncoding element may carry a byteOrder attribute specifying a value of 
/// a) bigEndian, the default, for values which are to be encoded most significant byte first; or 
/// b) littleEndian for values which are to be encoded least significant byte first. 
/// NOTE – The littleEndian specification applies only to data types whose size is a multiple of 8 bits.
#[test]
fn test_3_7_2() {
    // NOTE: byte order is not a resolved expression
    let data_type_set = get_test_data_type_set();

    for data_type in data_type_set.data_types {
        match data_type {
            DataType::FloatDataType(data) => {
                let data_string = data.encoding.unwrap().byte_order;
                assert!(data_string == "bigEndian".to_string() || data_string == "littleEndian".to_string())
            },
            DataType::IntegerDataType(data) => {
                let data_string = data.encoding.unwrap().byte_order;
                assert!(data_string == "bigEndian".to_string() || data_string == "littleEndian".to_string())
            },
            _ => (),
        }
    }
}

/// 3.7.3 A BooleanDataEncoding element shall carry a sizeInBits attribute which specifies the size, in bits, 
/// of the encoded data as a positive integer.
#[test]
fn test_3_7_3() {
    // NOTE: sizeInBites is not a resolved expression
    let data_type_set = get_test_data_type_set();

    for data_type in data_type_set.data_types {
        if let DataType::BooleanDataType(data) = data_type {
            let size_in_bits_string = data.encoding.unwrap().size_in_bits;
            let size_in_bits_int = size_in_bits_string.parse::<i32>().unwrap();
            assert!(size_in_bits_int > 0);
        }
    }
}

/// 3.7.10 A StringDataType shall carry a length attribute which defines the maximum possible length of the string, in bytes.
#[test]
fn test_3_7_10() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::StringDataType(data) = data_type {
            // convert length string to integer
            let length_str = data.length;
            let length_int = length_str.parse::<i32>().unwrap();
            assert!(length_int > 0);
        }
    }
}

/// 3.7.11 A StringDataType may carry a fixedLength attribute which, if ‘false’, indicates that the string can be shorter than the value specified by the length attribute.
#[test]
fn test_3_7_11() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::StringDataType(data) = data_type {
            if let Some(fixed_length) = data.fixed_length {
                assert!(fixed_length == "true".to_string() || fixed_length == "false".to_string());
            }
        }
    }
}

/// 3.7.12 A StringDataEncoding element may carry an encoding attribute which has a value of either a) UTF-8, specifying Unicode UTF-8 encoding; or b) ASCII, the default, specifying US ASCII encoding.
#[test]
fn test_3_7_12() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::StringDataType(data) = data_type {
            if let Some(encoding) = data.encoding {
                let encoding_string = encoding.encoding.unwrap();
                assert!(encoding_string == "UTF8".to_string() || encoding_string == "ASCII".to_string());
            }
        }
    }
}

/// 3.7.13 The optional terminationCharacter attribute of a StringDataEncoding element shall specify the termination character for the string.
#[test]
fn test_3_7_13() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::StringDataType(data) = data_type {
            assert!(data.encoding.unwrap().termination_character.is_some());
        }
    }
}

/// 3.7.14 An EnumeratedDataType shall contain an EnumerationList element, consisting of a list of one or more Enumeration elements.
#[test]
fn test_3_7_14() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::EnumeratedDataType(data) = data_type {
            assert!(data.enumeration_list.enumeration.len() > 0);
        }
    }
}

/// 3.7.15 Each Enumeration element shall have required label and value attributes, indicating the integer value corresponding to a given label string.
#[test]
fn test_3_7_15() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::EnumeratedDataType(data) = data_type {
            for enum_element in data.enumeration_list.enumeration {
                assert!(enum_element.label.len() > 0 && enum_element.value.len() > 0);
            }
        }
    }
}
