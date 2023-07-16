//! EDS PackageFile Model
use serde::{Deserialize, Serialize};

/// Package File describes a composable unit of software or hardware
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PackageFile {
    /// PackageFile includes a Package element  
    #[serde(rename = "Package", default)]
    pub package: Vec<Package>,
}

/// Package describes a related set of components, data types, and interfaces
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,

    /// A Package element may contain a DataTypeSet element
    #[serde(rename = "DataTypeSet", default)]
    pub data_type_set: DataTypeSet,
}

/// DataTypeSet element contains one or more DataType elements
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct DataTypeSet {
    /// DataTypeSet includes a DataType element
    pub data_types: Vec<DataType>,
}

/// DataTypeSet element contains one or more of the following elements:
/// ArrayDataType, BinaryDataType, BooleanDataType, ContainerDataType,
/// EnumeratedDataType, FloatDataType, IntegerDataType, StringDataType,
/// and SubRangeDataType.
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

/// EnumeratedDataType defines an enumerated data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumeratedDataType {
    #[serde(flatten)]
    pub name_field_type: NamedEntityType,
    #[serde(rename = "IntegerDataEncoding", default)]
    pub integer_data_encoding: IntegerDataEncoding,
    #[serde(rename = "EnumerationList", default)]
    pub enumeration_list: EnumerationList,
}

/// NamedEntityType stores the name attribute and may have the optional
/// shortDescription attribute and LongDescription child element.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NamedEntityType {
    pub name: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: Option<String>,
    #[serde(rename = "longDescription", default)]
    pub long_description: Option<String>,
}

///EnumerationList consists of a list of one or more Enumeration elements
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumerationList {
    #[serde(rename = "Enumeration", default)]
    pub enumeration: Vec<Enumeration>,
}

/// Enumeration element has required label and value attributes,
/// indicating the integer value corresponding to a given label string
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Enumeration {
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: String,
}

/// ContainerDataType defines a container data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ContainerDataType {
    #[serde(flatten)]
    pub name_field_type: NamedEntityType,
    #[serde(rename = "EntryList", default)]
    pub entry_list: EntryList,
}

/// EntryList consists of a list of one or more EntryElement elements
#[derive(Debug, Default, Serialize, PartialEq)]
pub struct EntryList {
    pub entries: Vec<EntryElement>,
}

/// EntryElement is either an Entry or a PaddingEntry
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryElement {
    Entry(Entry),
    PaddingEntry(PaddingEntry),
    LengthEntry(LengthEntry),
    ErrorControlEntry(ErrorControlEntry),
    FixedValueEntry(FixedValueEntry),
}

/// Entry element defines a field within a container
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    #[serde(flatten)]
    name_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,
}

/// PaddingEntry within a container has an attribute sizeInBits that specifies
/// the position of successive fields
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PaddingEntry {
    #[serde(rename = "sizeInBits")]
    pub size_in_bits: u32,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
}

/// ArrayDataType defines an array data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ArrayDataType {
    #[serde(flatten)]
    pub name_field_type: NamedEntityType,
    #[serde(rename = "dataTypeRef", default)]
    pub data_type_ref: String,
    #[serde(rename = "DimensionList", default)]
    pub dimension_list: DimensionList,
}

/// DimensionList consists of a list of one or more Dimension elements
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DimensionList {
    #[serde(rename = "Dimension", default)]
    pub dimension: Vec<Dimension>,
}

/// Dimension determines the length of the array dimension
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Dimension {
    #[serde(rename = "size", default)]
    pub size: String,
}

/// BooleanDataType defines a boolean data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BooleanDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "BooleanDataEncoding")]
    pub boolean_data_encoding: Option<BooleanDataEncoding>,
}

/// BooleanDataEncoding defines the size in bits of a boolean data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BooleanDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: u8,
}

/// IntegerDataType defines an integer data type
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

/// IntegerDataEncoding defines the encoding of an integer data type,
/// including the size in bits, encoding, and byte order
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct IntegerDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: String,
    #[serde(rename = "encoding", default)]
    pub encoding: String,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: String,
}

/// Range defines an interval of inclusive or exclusive minimum and maximum values
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "MinMaxRange", default)]
    pub min_max_range: MinMaxRange,
}

/// MinMaxRange defines the minimum and maximum values of a data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MinMaxRange {
    #[serde(rename = "max", default)]
    pub max: String,
    #[serde(rename = "min", default)]
    pub min: String,
    #[serde(rename = "rangeType", default)]
    pub range_type: String,
}

/// FloatDataEncoding defines the precision and encoding of a floating point data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FloatDataEncoding {
    #[serde(rename = "encodingAndPrecision", default)]
    pub encoding_and_precision: String,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: String,
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: u8,
}

/// FloatDataType defines a floating point data type
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FloatDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "FloatDataEncoding")]
    pub float_data_encoding: FloatDataEncoding,
    pub range: Option<Range>,
}

/// StringDataType defines a string data type of either fixed or variable length
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StringDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    pub length: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ComponentSet {
    #[serde(rename = "Component", default)]
    pub components: Vec<Component>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Component {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "RequiredInterfaceSet", default)]
    pub required_interface_set: RequiredInterfaceSet,
    #[serde(rename = "Implementation")]
    pub implementation: Implementation,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RequiredInterfaceSet {
    #[serde(rename = "Interface", default)]
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Interface {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "GenericTypeMapSet", default)]
    pub generic_type_map_set: GenericTypeMapSet,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GenericTypeMapSet {
    #[serde(rename = "GenericTypeMap", default)]
    pub generic_type_maps: Vec<GenericTypeMap>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GenericTypeMap {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Implementation {
    #[serde(rename = "VariableSet", default)]
    pub variable_set: VariableSet,
    #[serde(rename = "ParameterMapSet", default)]
    pub parameter_map_set: ParameterMapSet,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct VariableSet {
    #[serde(rename = "Variable", default)]
    pub variables: Vec<Variable>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Variable {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "initialValue")]
    pub initial_value: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ParameterMapSet {
    #[serde(rename = "ParameterMap", default)]
    pub parameter_maps: Vec<ParameterMap>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ParameterMap {
    #[serde(rename = "interface")]
    pub interface: String,
    #[serde(rename = "parameter")]
    pub parameter: String,
    #[serde(rename = "variableRef")]
    pub variable_ref: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LengthEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "PolynomialCalibrator")]
    pub polynomial_calibrator: PolynomialCalibrator,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PolynomialCalibrator {
    #[serde(rename = "Term")]
    pub term: Vec<Term>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Term {
    pub coefficient: String,
    pub exponent: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ErrorControlEntry {
    #[serde(flatten)]
    pub named_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "errorControlType")]
    pub error_control_type: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FixedValueEntry {
    #[serde(flatten)]
    pub named_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fixedValue")]
    pub fixed_value: String,
}
