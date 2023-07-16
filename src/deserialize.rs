//! Deserialization of EDS Models
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::datasheet::{DataType, DataTypeSet, EntryElement, EntryList};

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

struct EntryElementVisitor;

impl<'de> Visitor<'de> for EntryElementVisitor {
    type Value = Vec<EntryElement>;

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
                "Entry" => {
                    data_types.push(EntryElement::Entry(map.next_value()?));
                }
                "PaddingEntry" => {
                    data_types.push(EntryElement::PaddingEntry(map.next_value()?));
                }
                _ => return Err(de::Error::unknown_field(&key, &[])),
            }
        }

        Ok(data_types)
    }
}

impl<'de> Deserialize<'de> for EntryList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_map(EntryElementVisitor)
            .map(|entry_types| EntryList {
                entries: entry_types,
            })
    }
}
