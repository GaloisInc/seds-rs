//! Raw EDS PackageFile Model
use serde::{Deserialize, Serialize};

type Expression = String;

/// DataSheet contains one Device element and one or more Package elements
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DataSheet {
    #[serde(rename = "Device", default)]
    pub devices: Vec<Device>,

    #[serde(rename = "Package", default)]
    pub packages: Vec<Package>,
}

/// Device defines a device and is based on the NamedEntityType
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Device {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,

    #[serde(rename = "MetaData", default)]
    pub metadata: Option<MetaData>,
}

/// MetaData provides additional information about the Device or PackageFile
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MetaData {
    #[serde(rename = "CreationDate", default)]
    pub creation_date: Option<String>, // assuming creation date is string, change type accordingly

    #[serde(rename = "Creator", default)]
    pub creator: Option<String>,
    // Add other fields as required...
}

/// Package File describes a composable unit of software or hardware
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PackageFile {
    /// PackageFile includes a Package element  
    #[serde(rename = "Package", default)]
    pub package: Vec<Package>,
}

/// Package describes a related set of components, data types, and interfaces
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Package {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,

    /// A Package element may contain a DataTypeSet element
    #[serde(rename = "DataTypeSet", default)]
    pub data_type_set: Option<DataTypeSet>,

    #[serde(rename = "MetaData", default)]
    pub metadata: Option<MetaData>,
}

/// DataTypeSet element contains one or more DataType elements
#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct DataTypeSet {
    /// DataTypeSet includes a DataType element
    pub data_types: Vec<DataType>,
}

/// DataTypeSet element contains one or more of the following elements:
/// ArrayDataType, BinaryDataType, BooleanDataType, ContainerDataType,
/// EnumeratedDataType, FloatDataType, IntegerDataType, StringDataType,
/// and SubRangeDataType.
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub enum DataType {
    #[default]
    NoneDataType,
    BooleanDataType(BooleanDataType),
    IntegerDataType(IntegerDataType),
    ArrayDataType(ArrayDataType),
    EnumeratedDataType(EnumeratedDataType),
    ContainerDataType(ContainerDataType),
    FloatDataType(FloatDataType),
    StringDataType(StringDataType),
    SubRangeDataType(SubRangeDataType),
}

/// EnumeratedDataType defines an enumerated data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnumeratedDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "IntegerDataEncoding", default)]
    pub encoding: Option<IntegerDataEncoding>,
    #[serde(rename = "EnumerationList", default)]
    pub enumeration_list: EnumerationList,
}

/// NamedEntityType stores the name attribute and may have the optional
/// shortDescription attribute and LongDescription child element.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NamedEntityType {
    pub name: String,
    #[serde(rename = "shortDescription", default)]
    pub short_description: Option<String>,
    #[serde(rename = "LongDescription", default)]
    pub long_description: Option<LongDescription>,
}

/// LongDescription element contains text representing a long description
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LongDescription {
    #[serde(rename = "$value", default)]
    pub text: String,
}

///EnumerationList consists of a list of one or more Enumeration elements
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnumerationList {
    #[serde(rename = "Enumeration", default)]
    pub enumeration: Vec<Enumeration>,
}

/// Enumeration element has required label and value attributes,
/// indicating the integer value corresponding to a given label string
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Enumeration {
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "value", default)]
    pub value: Expression,
    #[serde(rename = "shortDescription", default)]
    pub short_description: Option<String>,
}

/// ContainerDataType defines a container data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContainerDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "EntryList", default)]
    pub entry_list: Option<EntryList>,
    #[serde(rename = "abstract")]
    pub _abstract: Option<Expression>,
    #[serde(rename = "baseType")]
    pub base_type: Option<String>,
    #[serde(rename = "ConstraintSet")]
    pub constraint_set: Option<ConstraintSet>,
    #[serde(rename = "TrailerEntryList")]
    pub trailer_entry_list: Option<EntryList>,
}

/// ConstraintSet specifies the criteria that apply to the entries of the container type
#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct ConstraintSet {
    pub constraints: Vec<Constraint>,
}

/// Constraint specifies the criteria that apply to the entries of the container type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Constraint {
    RangeConstraint(RangeConstraint),
    TypeConstraint(TypeConstraint),
    ValueConstraint(ValueConstraint),
}

/// RangeConstraint specifies the range of valid values for a container entry
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RangeConstraint {
    #[serde(rename = "Range", default)]
    pub range: Range,
    pub entry: Expression,
}

/// TypeConstraint specifies the data type of a container entry
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TypeConstraint {
    #[serde(rename = "type")]
    pub type_: String,
    pub entry: Expression,
}

/// ValueConstraint specifies the value of a container entry
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ValueConstraint {
    #[serde(rename = "value")]
    pub value: Expression,
    pub entry: Expression,
}

/// EntryList consists of a list of one or more EntryElement elements
#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct EntryList {
    pub entries: Vec<EntryElement>,
}

/// EntryElement is either an Entry or a PaddingEntry
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EntryElement {
    Entry(Entry),
    FixedValueEntry(FixedValueEntry),
    PaddingEntry(PaddingEntry),
    ListEntry(ListEntry),
    LengthEntry(LengthEntry),
    ErrorControlEntry(ErrorControlEntry),
}

/// Entry element defines a field within a container
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Entry {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,
}

/// PaddingEntry within a container has an attribute sizeInBits that specifies
/// the position of successive fields
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaddingEntry {
    #[serde(rename = "sizeInBits")]
    pub size_in_bits: Expression,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
}

/// ArrayDataType defines an array data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ArrayDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "dataTypeRef", default)]
    pub data_type_ref: String,
    #[serde(rename = "DimensionList", default)]
    pub dimension_list: DimensionList,
}

/// DimensionList consists of a list of one or more Dimension elements
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DimensionList {
    #[serde(rename = "Dimension", default)]
    pub dimension: Vec<Dimension>,
}

/// Dimension determines the length of the array dimension
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Dimension {
    #[serde(rename = "size", default)]
    pub size: Expression,
}

/// BooleanDataType defines a boolean data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BooleanDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "BooleanDataEncoding")]
    pub encoding: Option<BooleanDataEncoding>,
}

/// BooleanDataEncoding defines the size in bits of a boolean data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BooleanDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: Expression,
    #[serde(rename = "falseValue", default)]
    pub false_value: Option<Expression>,
}

/// BooleanFalseValue - Req 3.7.4
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum BooleanFalseValue {
    #[default]
    #[serde(rename = "zeroIsFalse")]
    ZeroIsFalse,
    #[serde(rename = "nonZeroIsFalse")]
    NonZeroIsFalse,
}

/// IntegerDataType defines an integer data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct IntegerDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "IntegerDataEncoding")]
    pub encoding: Option<IntegerDataEncoding>,
    #[serde(rename = "Range", default)]
    pub range: Range,
}

/// IntegerDataEncoding defines the encoding of an integer data type,
/// including the size in bits, encoding, and byte order
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct IntegerDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: Expression,
    #[serde(rename = "encoding", default)]
    pub encoding: Expression,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: Expression,
}

/// Range defines an interval of inclusive or exclusive minimum and maximum values
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Range {
    #[serde(rename = "MinMaxRange", default)]
    pub min_max_range: MinMaxRange,
}

/// MinMaxRange defines the minimum and maximum values of a data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MinMaxRange {
    #[serde(rename = "max", default)]
    pub max: Expression,
    #[serde(rename = "min", default)]
    pub min: Expression,
    #[serde(rename = "rangeType", default)]
    pub range_type: Expression,
}

/// FloatDataType defines a floating point data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FloatDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "FloatDataEncoding")]
    pub encoding: Option<FloatDataEncoding>,
    pub range: Option<Range>,
}

/// FloatDataEncoding defines the encoding of a floating point data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FloatDataEncoding {
    #[serde(rename = "sizeInBits", default)]
    pub size_in_bits: Expression,
    #[serde(rename = "byteOrder", default)]
    pub byte_order: Expression,
    #[serde(rename = "encodingAndPrecision", default)]
    pub encoding_and_precision: Expression,
}

/// StringDataType defines a string data type of either fixed or variable length
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StringDataType {
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    pub length: Expression,
    #[serde(rename = "StringDataEncoding")]
    pub encoding: Option<StringDataEncoding>,
    pub fixed_length: Option<Expression>,
}

/// StringDataEncoding defines the encoding of a string data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StringDataEncoding {
    #[serde(rename = "terminationCharacter")]
    pub termination_character: Option<Expression>,
    pub encoding: Option<Expression>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComponentSet {
    #[serde(rename = "Component", default)]
    pub components: Vec<Component>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Component {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "RequiredInterfaceSet", default)]
    pub required_interface_set: RequiredInterfaceSet,
    #[serde(rename = "Implementation")]
    pub implementation: Implementation,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequiredInterfaceSet {
    #[serde(rename = "Interface", default)]
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Interface {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    #[serde(rename = "GenericTypeMapSet", default)]
    pub generic_type_map_set: GenericTypeMapSet,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericTypeMapSet {
    #[serde(rename = "GenericTypeMap", default)]
    pub generic_type_maps: Vec<GenericTypeMap>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericTypeMap {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Implementation {
    #[serde(rename = "VariableSet", default)]
    pub variable_set: VariableSet,
    #[serde(rename = "ParameterMapSet", default)]
    pub parameter_map_set: ParameterMapSet,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct VariableSet {
    #[serde(rename = "Variable", default)]
    pub variables: Vec<Variable>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Variable {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "initialValue")]
    pub initial_value: Expression,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ParameterMapSet {
    #[serde(rename = "ParameterMap", default)]
    pub parameter_maps: Vec<ParameterMap>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ParameterMap {
    #[serde(rename = "interface")]
    pub interface: String,
    #[serde(rename = "parameter")]
    pub parameter: String,
    #[serde(rename = "variableRef")]
    pub variable_ref: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LengthEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    #[serde(rename = "PolynomialCalibrator")]
    pub polynomial_calibrator: Option<PolynomialCalibrator>,
}

/// PolynomialCalibrator calibration that would be required to take the raw value represented by the data
/// type and convert it into the units and other semantic terms associated with the field
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PolynomialCalibrator {
    #[serde(rename = "Term")]
    pub term: Vec<Term>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Term {
    pub coefficient: String,
    pub exponent: String,
}

/// ErrorControlEntry specifies an entry whose value is constrained, or derived,
/// based on the contents of the container in which it is present.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorControlEntry {
    #[serde(flatten)]
    pub named_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "errorControlType")]
    pub error_control_type: Expression,
}

/// FixedValueEntry within a container contains a fixed value
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FixedValueEntry {
    #[serde(flatten)]
    pub named_entity_type: NamedEntityType,
    #[serde(rename = "type")]
    pub type_: String,

    /// value to which the container entry should be fixed
    /// the value is a Literal whose type matches the type of the entry
    /// TODO: table 3.1
    #[serde(rename = "fixedValue")]
    pub fixed_value: String,
}

/// TODO: ListEntry
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListEntry {
    // TODO
}

/// SubRangeDataType defines a sub range data type
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SubRangeDataType {
    #[serde(rename = "baseType")]
    pub base_type: String,
    #[serde(flatten)]
    pub name_entity_type: NamedEntityType,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "Range", default)]
    pub range: Range,
    pub encoding: Option<Expression>,
}
