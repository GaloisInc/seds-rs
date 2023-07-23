//! EDS PackageFile Model

/// Identifier
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Identifier(pub String);

/// Qualified name
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName(pub String);

/// Literal Encoding
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Literal(pub String);

/// IntegerEncoding - Req 3.7.5
#[derive(Debug, Default, Clone, PartialEq)]
pub enum IntegerEncoding {
    #[default]
    Unsigned,
    SignMagnitude,
    TwosComplement,
    OnesComplement,
    BinaryCodedDecimal,
}

/// StringEncoding - Req 3.7.12
#[derive(Debug, Default, Clone, PartialEq)]
pub enum StringEncoding {
    #[default]
    ASCII,
    UTF8,
}

/// MinMaxRangeType Options - Table 3.2
#[derive(Debug, Default, Clone, PartialEq)]
pub enum MinMaxRangeType {
    /// {x | a < x < b}
    #[default]
    ExclusiveMinExclusiveMax,

    /// {x | a <= x <= b}
    InclusiveMinInclusiveMax,

    /// {x | a <= x < b}
    InclusiveMinExclusiveMax,

    /// {x | a < x <= b}
    ExclusiveMinInclusiveMax,

    /// {x | a < x}
    GreaterThan,

    /// {x | a <= x}
    AtLeast,

    /// {x | x < b}
    LessThan,

    /// {x | x <= b}
    AtMost,
}

/// FloatDataEncoding defines the precision and encoding of a floating point data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatDataEncoding {
    pub encoding_and_precision: FloatEncodingAndPrecision,
    pub byte_order: ByteOrder,
    pub size_in_bits: usize,
}

/// FloatEncodingAndPrecision defines the encoding and precision of a floating point data type
#[derive(Debug, Default, Clone, PartialEq)]
pub enum FloatEncodingAndPrecision {
    #[default]
    IEEE7542008Single,
    IEEE7542008Double,
    IEEE7542008Quadruple,
    MILSTD1770ASimple,
    MILSTD1770AExtended,
}

/// ByteOrder defines the byte order of a data type
#[derive(Debug, Default, Clone, PartialEq)]
pub enum ByteOrder {
    #[default]
    BigEndian,
    LittleEndian,
}

/// ErrorControlType - Table 3.3
#[derive(Debug, Default, Clone, PartialEq)]
pub enum ErrorControlType {
    /// G(X) = X^16 + X^12 + X^5 + 1
    #[default]
    CRC16CCITT,
    /// G(x) = x^8 + x^2 + x^1 + x^0
    CRC8,
    /// modulo 2^32 addition of all 4-byte
    CHECKSUM,
    /// Longitudinal redundancy check, bitwise XOR of all bytes
    CHECKSUMLONGITUDINAL,
}

/// Package File describes a composable unit of software or hardware
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PackageFile {
    /// PackageFile includes a Package element  
    pub package: Vec<Package>,
}

/// Package describes a related set of components, data types, and interfaces
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Package {
    pub name_entity_type: NamedEntityType,

    /// A Package element may contain a DataTypeSet element
    pub data_type_set: DataTypeSet,

    /// A Package element may contain a MetaData element
    pub metadata: Option<MetaData>,
}

/// MetaData provides additional information about the Device or PackageFile
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MetaData {
    pub creation_date: Option<String>,

    pub creator: Option<String>,
}

/// DataTypeSet element contains one or more DataType elements
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DataTypeSet {
    /// DataTypeSet includes a DataType element
    pub data_types: Vec<DataType>,
}

/// DataTypeSet element contains one or more of the following elements:
/// ArrayDataType, BinaryDataType, BooleanDataType, ContainerDataType,
/// EnumeratedDataType, FloatDataType, IntegerDataType, StringDataType,
/// and SubRangeDataType.
#[derive(Debug, Default, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumeratedDataType {
    pub name_field_type: NamedEntityType,
    pub encoding: IntegerDataEncoding,
    pub enumeration_list: EnumerationList,
}

/// NamedEntityType stores the name attribute and may have the optional
/// shortDescription attribute and LongDescription child element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NamedEntityType {
    pub name: Identifier,
    pub short_description: Option<String>,
    pub long_description: Option<LongDescription>,
}

/// LongDescription element contains text representing a long description
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LongDescription {
    pub text: String,
}

///EnumerationList consists of a list of one or more Enumeration elements
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumerationList {
    pub enumeration: Vec<Enumeration>,
}

/// Enumeration element has required label and value attributes,
/// indicating the integer value corresponding to a given label string
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enumeration {
    pub label: Identifier,
    pub value: Literal,
    pub short_description: String,
}

/// ContainerDataType defines a container data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContainerDataType {
    pub name_entity_type: NamedEntityType,
    pub entry_list: Option<EntryList>,
    pub _abstract: bool,
    pub base_type: Option<String>,
    pub constraint_set: Option<ConstraintSet>,
    pub trailer_entry_list: Option<EntryList>,
}

/// ConstraintSet specifies the criteria that apply to the entries of the container type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConstraintSet {
    pub constraints: Vec<Constraint>,
}

/// Constraint specifies the criteria that apply to the entries of the container type
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    RangeConstraint(RangeConstraint),
    TypeConstraint(TypeConstraint),
    ValueConstraint(ValueConstraint),
}

/// RangeConstraint specifies the range of valid values for a container entry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RangeConstraint {
    pub range: Range,
    pub entry: Identifier,
}

/// TypeConstraint specifies the data type of a container entry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypeConstraint {
    pub type_: QualifiedName,
    pub entry: Identifier,
}

/// ValueConstraint specifies the value of a container entry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ValueConstraint {
    pub value: Literal,
    pub entry: Identifier,
}

/// EntryList consists of a list of one or more EntryElement elements
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EntryList {
    pub entries: Vec<EntryElement>,
}

/// EntryElement is either an Entry or a PaddingEntry
#[derive(Debug, Clone, PartialEq)]
pub enum EntryElement {
    Entry(Entry),
    FixedValueEntry(FixedValueEntry),
    PaddingEntry(PaddingEntry),
    ListEntry(ListEntry),
    LengthEntry(LengthEntry),
    ErrorControlEntry(ErrorControlEntry),
}

/// Entry element defines a field within a container
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Entry {
    pub name_entity_type: NamedEntityType,
    pub type_: QualifiedName,
}

/// PaddingEntry within a container has an attribute sizeInBits that specifies
/// the position of successive fields
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PaddingEntry {
    pub size_in_bits: usize,
    pub short_description: Option<String>,
}

/// ArrayDataType defines an array data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrayDataType {
    pub name_field_type: NamedEntityType,
    pub data_type_ref: QualifiedName,
    pub dimension_list: DimensionList,
}

/// DimensionList consists of a list of one or more Dimension elements
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DimensionList {
    pub dimension: Vec<Dimension>,
}

/// Dimension determines the length of the array dimension
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dimension {
    pub size: usize,
}

/// BooleanDataType defines a boolean data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BooleanDataType {
    pub name_entity_type: NamedEntityType,
    pub encoding: BooleanDataEncoding,
}

/// BooleanDataEncoding defines the size in bits of a boolean data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BooleanDataEncoding {
    pub size_in_bits: usize,
    pub false_value: bool,
}

/// BooleanFalseValue - Req 3.7.4
#[derive(Debug, Default, Clone, PartialEq)]
pub enum BooleanFalseValue {
    #[default]
    ZeroIsFalse,
    NonZeroIsFalse,
}

/// IntegerDataType defines an integer data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IntegerDataType {
    pub name_entity_type: NamedEntityType,
    pub encoding: IntegerDataEncoding,
    pub range: Range,
}

/// IntegerDataEncoding defines the encoding of an integer data type,
/// including the size in bits, encoding, and byte order
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IntegerDataEncoding {
    pub size_in_bits: usize,
    pub encoding: IntegerEncoding,
    pub byte_order: ByteOrder,
}

/// Range defines an interval of inclusive or exclusive minimum and maximum values
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Range {
    pub min_max_range: MinMaxRange,
}

/// MinMaxRange defines the minimum and maximum values of a data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MinMaxRange {
    pub max: Literal,
    pub min: Literal,
    pub range_type: MinMaxRangeType,
}

/// FloatDataType defines a floating point data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatDataType {
    pub name_entity_type: NamedEntityType,
    pub encoding: FloatDataEncoding,
    pub range: Option<Range>,
}

/// StringDataType defines a string data type of either fixed or variable length
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringDataType {
    pub name_entity_type: NamedEntityType,
    pub length: usize,
    pub encoding: StringDataEncoding,
    pub fixed_length: bool,
}

/// StringDataEncoding defines the encoding of a string data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringDataEncoding {
    pub termination_character: Option<char>,
    pub encoding: StringEncoding,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ComponentSet {
    pub components: Vec<Component>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Component {
    pub name: Identifier,
    pub required_interface_set: RequiredInterfaceSet,
    pub implementation: Implementation,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RequiredInterfaceSet {
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Interface {
    pub name: Identifier,
    pub type_: QualifiedName,
    pub short_description: String,
    pub generic_type_map_set: GenericTypeMapSet,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GenericTypeMapSet {
    pub generic_type_maps: Vec<GenericTypeMap>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GenericTypeMap {
    pub name: Identifier,
    pub type_: QualifiedName,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Implementation {
    pub variable_set: VariableSet,
    pub parameter_map_set: ParameterMapSet,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VariableSet {
    pub variables: Vec<Variable>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Variable {
    pub type_: QualifiedName,
    pub read_only: bool,
    pub name: Identifier,
    pub initial_value: Literal,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ParameterMapSet {
    pub parameter_maps: Vec<ParameterMap>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ParameterMap {
    pub interface: String,
    pub parameter: String,
    pub variable_ref: QualifiedName,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LengthEntry {
    pub name_entity_type: NamedEntityType,
    pub type_: QualifiedName,
    pub calibration: Option<PolynomialCalibrator>,
}

/// PolynomialCalibrator calibration that would be required to take the raw value represented by the data
/// type and convert it into the units and other semantic terms associated with the field
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PolynomialCalibrator {
    pub term: Vec<Term>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Term {
    pub coefficient: Literal,
    pub exponent: Literal,
}

/// ErrorControlEntry specifies an entry whose value is constrained, or derived,
/// based on the contents of the container in which it is present.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ErrorControlEntry {
    pub name_entity_type: NamedEntityType,
    pub type_: QualifiedName,
    pub error_control_type: ErrorControlType,
}

/// FixedValueEntry within a container contains a fixed value
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FixedValueEntry {
    pub name_entity_type: NamedEntityType,
    pub type_: QualifiedName,

    /// value to which the container entry should be fixed
    /// the value is a Literal whose type matches the type of the entry
    /// TODO: table 3.1
    pub fixed_value: Literal,
}

/// TODO: ListEntry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListEntry {
    pub name_entity_type: NamedEntityType,
    pub list_length_field: usize,
}

/// SubRangeDataType defines a sub range data type
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SubRangeDataType {
    pub base_type: String,
    pub name_entity_type: NamedEntityType,
    pub unit: String,
    pub range: Range,
}
