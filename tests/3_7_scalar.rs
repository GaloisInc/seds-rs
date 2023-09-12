//! 3.7 SCALAR DATA TYPES
use seds_rs::eds::raw::{DataType, DataTypeSet, PackageFile};

mod common;

use common::open_file;

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
                let data_string = data.encoding.unwrap().byte_order.unwrap();
                assert!(
                    data_string == *"bigEndian"
                        || data_string == *"littleEndian"
                )
            }
            DataType::IntegerDataType(data) => {
                let data_string = data.encoding.unwrap().byte_order;
                assert!(
                    data_string == Some("bigEndian".to_string())
                        || data_string == Some("littleEndian".to_string())
                )
            }
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

/// 3.7.4 A BooleanDataEncoding element may carry a falseValue attribute which specifies the value that corresponds to logical falsehood, with options a) zeroIsFalse (the default); and b) nonZeroIsFalse.
#[test]
fn test_3_7_4() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::BooleanDataType(data) = data_type {
            let value = data.encoding.unwrap().false_value.unwrap();
            assert!(value == "zeroIsFalse" || value == "nonZeroIsFalse");
        }
    }
}

/// 3.7.5 An IntegerDataEncoding element shall carry an encoding attribute which has a value of a) unsigned, for an unsigned value; b) signMagnitude, for an encoding with a separate sign bit (the most significant bit is the sign bit, with 1 indicating negative); c) twosComplement, for twos complement; d) onesComplement, for ones complement; e) BCD, for a natural unsigned binary coded decimal, where each byte is a decimal digit encoded as binary; or f) packedBCD, where each byte contains two decimal digits encoded as binary, followed by an optional sign nibble.
#[test]
fn test_3_7_5() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::IntegerDataType(data) = data_type {
            let value = data.encoding.unwrap().encoding;
            let value_str = value.as_str();
            assert!(matches!(
                value_str,
                "unsigned"
                    | "signMagnitude"
                    | "twosComplement"
                    | "onesComplement"
                    | "BCD"
                    | "packedBCD"
            ));
        }
    }
}

/// 3.7.6 An IntegerDataEncoding element shall carry a sizeInBits attribute which specifies the size, in bits, of the encoded data as a positive integer.
#[test]
fn test_3_7_6() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::IntegerDataType(data) = data_type {
            let size_in_bits = data.encoding.unwrap().size_in_bits;
            let size_in_bits_int = size_in_bits.parse::<i32>().unwrap();
            assert!(size_in_bits_int > 0);
        }
    }
}

/// 3.7.7 The size in bits of a BCD encoding shall be a multiple of 8. The size in bits of a packedBCD shall be a multiple of 4. The size in bits of both forms of binary coded decimals is a fixed value, so all high-order digits that are zero shall be present to fill the fixed size in bits.
#[test]
fn test_3_7_7() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::IntegerDataType(data) = data_type {
            let encoding = data.encoding.unwrap();
            if matches!(encoding.encoding.as_str(), "BCD") {
                let size_in_bits = encoding.size_in_bits;
                let size_in_bits_int = size_in_bits.parse::<i32>().unwrap();
                assert!(size_in_bits_int % 8 == 0);
            } else if matches!(encoding.encoding.as_str(), "PackedBCD") {
                let size_in_bits = encoding.size_in_bits;
                let size_in_bits_int = size_in_bits.parse::<i32>().unwrap();
                assert!(size_in_bits_int % 4 == 0);
            }
        }
    }
}

/// 3.7.8 A FloatDataEncoding element shall carry an encodingAndPrecision attribute which has a value of either a) IEEE754_2008_single; b) IEEE754_2008_double; c) IEEE754_2008_quad; d) MILSTD_1750A_simple; or e) MILSTD_1750A_extended.
#[test]
fn test_3_7_8() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::FloatDataType(data) = data_type {
            let enc_and_prec = data.encoding.unwrap().encoding_and_precision;
            assert!(matches!(
                enc_and_prec.as_str(),
                "IEEE754_2008_single"
                    | "IEEE754_2008_double"
                    | "IEEE754_2008_quad"
                    | "MILSTD_1750A_simple"
                    | "MILSTD_1750A_extended"
            ));
        }
    }
}

/// 3.7.9 A FloatDataEncoding element shall carry a sizeInBits attribute which specifies the size, in bits, of the encoded data as a positive integer.
#[test]
fn test_3_7_9() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::FloatDataType(data) = data_type {
            let size_in_bits = data.encoding.unwrap().size_in_bits;
            let size_in_bits_int = size_in_bits.parse::<i32>().unwrap();
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
                assert!(fixed_length == *"true" || fixed_length == *"false");
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
                assert!(
                    encoding_string == *"UTF8" || encoding_string == *"ASCII"
                );
            }
        }
    }
}

/// 3.7.13 The optional terminationCharacter attribute of a StringDataEncoding element shall specify the termination character for the string.
// #[test]
// fn test_3_7_13() {
//     let data_type_set = get_test_data_type_set();
//     for data_type in data_type_set.data_types {
//         if let DataType::StringDataType(data) = data_type {
//             assert!(data.encoding.unwrap().termination_character.is_some());
//         }
//     }
// }

/// 3.7.14 An EnumeratedDataType shall contain an EnumerationList element, consisting of a list of one or more Enumeration elements.
#[test]
fn test_3_7_14() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::EnumeratedDataType(data) = data_type {
            assert!(!data.enumeration_list.enumeration.is_empty());
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
                assert!(!enum_element.label.is_empty() && !enum_element.value.is_empty());
            }
        }
    }
}
