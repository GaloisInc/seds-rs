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
struct DataTypeSet {
    #[serde(rename = "$value", default)]
    data_types: Vec<DataType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum DataType {
    EnumeratedDataType(EnumeratedDataType),
    ContainerDataType(ContainerDataType),
    ArrayDataType(ArrayDataType),
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumeratedDataType {
    #[serde(flatten)]
    name_field_type: NamedFieldType,
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
pub struct NamedFieldType {
    name: String,
    #[serde(rename = "shortDescription", default)]
    short_description: Option<String>,
    #[serde(rename = "longDescription", default)]
    long_description: Option<String>,
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

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct ContainerDataType {
    #[serde(flatten)]
    name_field_type: NamedFieldType,
    #[serde(rename = "EntryList", default)]
    entry_list: EntryList,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct EntryList {
    #[serde(rename = "Entry", default)]
    entry: Vec<Entry>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Entry {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "type", default)]
    entry_type: String,
    #[serde(rename = "shortDescription", default)]
    short_description: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct ArrayDataType {
    #[serde(flatten)]
    name_field_type: NamedFieldType,
    #[serde(rename = "dataTypeRef", default)]
    data_type_ref: String,
    #[serde(rename = "DimensionList", default)]
    dimension_list: DimensionList,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct DimensionList {
    #[serde(rename = "Dimension", default)]
    dimension: Vec<Dimension>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Dimension {
    #[serde(rename = "size", default)]
    size: String,
}
