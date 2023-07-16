use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_xml_rs::from_str;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PackageFile {
    #[serde(rename = "Package", default)]
    package: Vec<Package>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    #[serde(rename = "DataTypeSet", default)]
    data_type_set: DataTypeSet,
}

#[derive(Debug, Default, Serialize, PartialEq)]
struct DataTypeSet {
    data_types: Vec<DataType>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum DataType {
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
    name_field_type: NamedFieldType,
    #[serde(rename = "IntegerDataEncoding", default)]
    integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "EnumerationList", default)]
    enumeration_list: EnumerationList,
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
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    #[serde(rename = "type", default)]
    entry_type: String,
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

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct BooleanDataType {
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    #[serde(rename = "BooleanDataEncoding")]
    boolean_data_encoding: BooleanDataEncoding,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct BooleanDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    size_in_bits: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct IntegerDataType {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "shortDescription", default)]
    short_description: String,
    #[serde(rename = "IntegerDataEncoding")]
    integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "Range", default)]
    range: Range,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct IntegerDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    size_in_bits: String,
    #[serde(rename = "encoding", default)]
    encoding: String,
    #[serde(rename = "byteOrder", default)]
    byte_order: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Range {
    #[serde(rename = "MinMaxRange", default)]
    min_max_range: MinMaxRange,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct MinMaxRange {
    #[serde(rename = "max", default)]
    max: String,
    #[serde(rename = "min", default)]
    min: String,
    #[serde(rename = "rangeType", default)]
    range_type: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct FloatDataEncoding {
    #[serde(rename = "encodingAndPrecision", default)]
    encoding_and_precision: String,
    #[serde(rename = "byteOrder", default)]
    byte_order: String,
    #[serde(rename = "sizeInBits", default)]
    size_in_bits: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct FloatDataType {
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    #[serde(rename = "FloatDataEncoding")]
    float_data_encoding: FloatDataEncoding,
    range: Option<Range>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct StringDataType {
    #[serde(flatten)]
    named_field_type: NamedFieldType,
    length: String,
}

struct DataTypeVisitor;

impl<'de> Visitor<'de> for DataTypeVisitor {
    type Value = Vec<DataType>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a DataTypeSet containing multiple types of data")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut data_types = Vec::new();

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "IntegerDataType" => {
                    data_types.push(DataType::IntegerDataType(map.next_value()?));
                }
                "BooleanDataType" => {
                    data_types.push(DataType::BooleanDataType(map.next_value()?));
                }
                "ContainerDataType" => {
                    data_types.push(DataType::ContainerDataType(map.next_value()?));
                }
                "EnumeratedDataType" => {
                    data_types.push(DataType::EnumeratedDataType(map.next_value()?));
                }
                "ArrayDataType" => {
                    data_types.push(DataType::ArrayDataType(map.next_value()?));
                }
                "FloatDataType" => {
                    data_types.push(DataType::FloatDataType(map.next_value()?));
                }
                "StringDataType" => {
                    data_types.push(DataType::StringDataType(map.next_value()?));
                }
                _ => return Err(de::Error::unknown_field(&key, &[])),
            }
        }

        Ok(data_types)
    }
}

impl<'de> Deserialize<'de> for DataTypeSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_map(DataTypeVisitor)
            .map(|data_types| DataTypeSet { data_types })
    }
}
