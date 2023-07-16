use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PackageFile {
    #[serde(rename = "Package", default)]
    package: Vec<Package>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "shortDescription", default)]
    short_description: String,
    #[serde(rename = "DataTypeSet", default)]
    data_type_set: DataTypeSet,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DataTypeSet {
    #[serde(rename = "EnumeratedDataType", default)]
    enumerated_data_type: Vec<EnumeratedDataType>,
    //#[serde(rename = "StringDataType", default)]
    //string_data_type: Vec<StringDataType>,
    //#[serde(rename = "ContainerDataType", default)]
    //container_data_type: Vec<ContainerDataType>,
    //#[serde(rename = "ArrayDataType", default)]
    //array_data_type: Vec<ArrayDataType>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumeratedDataType {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "shortDescription", default)]
    short_description: String,
    #[serde(rename = "IntegerDataEncoding", default)]
    integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "EnumerationList", default)]
    enumeration_list: EnumerationList,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct IntegerDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    size_in_bits: String,
    #[serde(rename = "encoding", default)]
    encoding: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumerationList {
    #[serde(rename = "Enumeration", default)]
    enumeration: Vec<Enumeration>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Enumeration {
    #[serde(rename = "label", default)]
    label: String,
    #[serde(rename = "value", default)]
    value: String,
    #[serde(rename = "shortDescription", default)]
    short_description: String,
}
