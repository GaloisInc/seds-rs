use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PackageFile {
    #[serde(rename = "Package", default)]
    pub package: Vec<Package>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(flatten)]
    pub named_field_type: NamedFieldType,
    #[serde(rename = "DataTypeSet", default)]
    pub data_type_set: DataTypeSet,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct DataTypeSet {
    pub data_types: Vec<DataType>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum DataType {
    BooleanDataType(BooleanDataType),
    IntegerDataType(IntegerDataType),
    ArrayDataType(ArrayDataType),
    EnumeratedDataType(EnumeratedDataType),
    ContainerDataType(ContainerDataType),
    FloatDataType(FloatDataType),
    StringDataType(StringDataType),
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumeratedDataType {
    #[serde(flatten)]
    pub name_field_type: NamedFieldType,
    #[serde(rename = "IntegerDataEncoding", default)]
    pub integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "EnumerationList", default)]
    pub enumeration_list: EnumerationList,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NamedFieldType {
    pub name: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: Option<String>,
    #[serde(rename = "longDescription", default)]
    pub long_description: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumerationList {
    #[serde(rename = "Enumeration", default)]
    pub enumeration: Vec<Enumeration>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Enumeration {
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ContainerDataType {
    #[serde(flatten)]
    pub name_field_type: NamedFieldType,
    #[serde(rename = "EntryList", default)]
    pub entry_list: EntryList,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct EntryList {
    pub entries: Vec<EntryElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryElement {
    Entry(Entry),
    PaddingEntry(PaddingEntry),
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PaddingEntry {
    #[serde(rename = "sizeInBits")]
    pub size_in_bits: u32,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ArrayDataType {
    #[serde(flatten)]
    pub name_field_type: NamedFieldType,
    #[serde(rename = "dataTypeRef", default)]
    pub data_type_ref: String,
    #[serde(rename = "DimensionList", default)]
    pub dimension_list: DimensionList,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DimensionList {
    #[serde(rename = "Dimension", default)]
    pub dimension: Vec<Dimension>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Dimension {
    #[serde(rename = "size", default)]
    pub size: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BooleanDataType {
    #[serde(flatten)]
    pub named_field_type: NamedFieldType,
    #[serde(rename = "BooleanDataEncoding")]
    pub boolean_data_encoding: BooleanDataEncoding,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BooleanDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct IntegerDataType {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: String,
    #[serde(rename = "IntegerDataEncoding")]
    pub integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "Range", default)]
    pub range: Range,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct IntegerDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: String,
    #[serde(rename = "encoding", default)]
    pub encoding: String,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "MinMaxRange", default)]
    pub min_max_range: MinMaxRange,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MinMaxRange {
    #[serde(rename = "max", default)]
    pub max: String,
    #[serde(rename = "min", default)]
    pub min: String,
    #[serde(rename = "rangeType", default)]
    pub range_type: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FloatDataEncoding {
    #[serde(rename = "encodingAndPrecision", default)]
    pub encoding_and_precision: String,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: String,
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FloatDataType {
    #[serde(flatten)]
    pub named_field_type: NamedFieldType,
    #[serde(rename = "FloatDataEncoding")]
    pub float_data_encoding: FloatDataEncoding,
    pub range: Option<Range>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StringDataType {
    #[serde(flatten)]
    pub named_field_type: NamedFieldType,
    pub length: String,
}
