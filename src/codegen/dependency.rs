//! Collect Dependencies for Codegen
use crate::eds::ast::*;

/// put all relevant ast items in an enum so we iterate over them
#[derive(Debug)]
pub enum AstNode<'a> {
    /// AstNode Reference to PackageFile
    PackageFile(&'a PackageFile),
    /// AstNode Reference to Package
    Package(&'a Package),
    /// AstNode Reference to NamedEntityType
    NamedEntityType(&'a NamedEntityType),
    /// AstNode Reference to EntryList
    EntryList(&'a EntryList),
    /// AstNode Reference to EntryElement
    EntryElement(&'a EntryElement),
    /// AstNode Reference to FixedValueEntry
    FixedValueEntry(&'a FixedValueEntry),
    /// AstNode Reference to ErrorControlEntry
    ErrorControlEntry(&'a ErrorControlEntry),
    /// AstNode Reference to Entry
    Entry(&'a Entry),
    /// AstNode Reference to LengthEntry
    LengthEntry(&'a LengthEntry),
    /// AstNode Reference to DataTypeSet
    DataTypeSet(&'a DataTypeSet),
    /// AstNode Reference to DataType
    DataType(&'a DataType),
    /// AstNode Reference to NoneDataType
    NoneDataType,
    /// AstNode Reference to BooleanDataType
    BooleanDataType(&'a BooleanDataType),
    /// AstNode Reference to IntegerDataType
    IntegerDataType(&'a IntegerDataType),
    /// AstNode Reference to ArrayDataType
    ArrayDataType(&'a ArrayDataType),
    /// AstNode Reference to EnumeratedDataType
    EnumeratedDataType(&'a EnumeratedDataType),
    /// AstNode Reference to ContainerDataType
    ContainerDataType(&'a ContainerDataType),
    /// AstNode Reference to FloatDataType
    FloatDataType(&'a FloatDataType),
    /// AstNode Reference to StringDataType
    StringDataType(&'a StringDataType),
    /// AstNode Reference to SubRangeDataType
    SubRangeDataType(&'a SubRangeDataType),
    /// AstNode Reference to DimensionList
    DimensionList(&'a DimensionList),
    /// AstNode Reference to Range
    Range(&'a Range),
    /// AstNode Reference to PaddingEntry
    PaddingEntry(&'a PaddingEntry),
    /// AstNode Reference to ListEntry
    ListEntry(&'a ListEntry),
}

/// iterate over qualified names in the ast
pub struct QualifiedNameIter<'a> {
    stack: Vec<AstNode<'a>>,
}

impl<'a> QualifiedNameIter<'a> {
    /// constructor
    pub fn new(root: AstNode<'a>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for QualifiedNameIter<'a> {
    type Item = &'a QualifiedName;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                AstNode::PackageFile(pkg_file) => {
                    for pkg in pkg_file.package.iter().rev() {
                        self.stack.push(AstNode::Package(pkg));
                    }
                }
                AstNode::Package(pkg) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&pkg.name_entity_type));
                    self.stack.push(AstNode::DataTypeSet(&pkg.data_type_set));
                }
                AstNode::DataTypeSet(dts) => {
                    for dt in dts.data_types.iter().rev() {
                        self.stack.push(AstNode::DataType(dt));
                    }
                }
                AstNode::DataType(datatype) => match datatype {
                    DataType::NoneDataType => self.stack.push(AstNode::NoneDataType),
                    DataType::BooleanDataType(dt) => self.stack.push(AstNode::BooleanDataType(dt)),
                    DataType::IntegerDataType(dt) => self.stack.push(AstNode::IntegerDataType(dt)),
                    DataType::ArrayDataType(dt) => self.stack.push(AstNode::ArrayDataType(dt)),
                    DataType::EnumeratedDataType(dt) => {
                        self.stack.push(AstNode::EnumeratedDataType(dt))
                    }
                    DataType::ContainerDataType(dt) => {
                        self.stack.push(AstNode::ContainerDataType(dt))
                    }
                    DataType::FloatDataType(dt) => self.stack.push(AstNode::FloatDataType(dt)),
                    DataType::StringDataType(dt) => self.stack.push(AstNode::StringDataType(dt)),
                    DataType::SubRangeDataType(dt) => {
                        self.stack.push(AstNode::SubRangeDataType(dt))
                    }
                },
                AstNode::ContainerDataType(cdt) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&cdt.name_entity_type));
                    match &cdt.entry_list {
                        Some(el) => self.stack.push(AstNode::EntryList(el)),
                        None => (),
                    }
                    match &cdt.base_type {
                        Some(bt) => return Some(bt),
                        None => (),
                    }
                }
                AstNode::EntryList(el) => {
                    for entry in el.entries.iter().rev() {
                        self.stack.push(AstNode::EntryElement(entry));
                    }
                }
                AstNode::EntryElement(ee) => match ee {
                    EntryElement::Entry(e) => self.stack.push(AstNode::Entry(e)),
                    EntryElement::LengthEntry(e) => self.stack.push(AstNode::LengthEntry(e)),
                    EntryElement::ErrorControlEntry(e) => {
                        self.stack.push(AstNode::ErrorControlEntry(e))
                    }
                    EntryElement::FixedValueEntry(e) => {
                        self.stack.push(AstNode::FixedValueEntry(e))
                    }
                    EntryElement::PaddingEntry(e) => self.stack.push(AstNode::PaddingEntry(e)),
                    EntryElement::ListEntry(e) => self.stack.push(AstNode::ListEntry(e)),
                },
                AstNode::Entry(e) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&e.name_entity_type));
                    return Some(&e.type_);
                }
                AstNode::LengthEntry(le) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&le.name_entity_type));
                    return Some(&le.type_);
                }
                AstNode::ErrorControlEntry(ece) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&ece.name_entity_type));
                    return Some(&ece.type_);
                }
                AstNode::FixedValueEntry(fve) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&fve.name_entity_type));
                    return Some(&fve.type_);
                }
                AstNode::ArrayDataType(adt) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&adt.name_entity_type));
                    self.stack.push(AstNode::DimensionList(&adt.dimension_list));
                    return Some(&adt.data_type_ref);
                }
                AstNode::SubRangeDataType(srdt) => {
                    self.stack
                        .push(AstNode::NamedEntityType(&srdt.name_entity_type));
                    self.stack.push(AstNode::Range(&srdt.range));
                    return Some(&srdt.base_type);
                }
                AstNode::IntegerDataType(_idt) => (),
                AstNode::BooleanDataType(_bdt) => (),
                AstNode::NamedEntityType(_net) => (),
                AstNode::NoneDataType => (),
                AstNode::FloatDataType(_net) => (),
                AstNode::EnumeratedDataType(_edt) => {}
                AstNode::StringDataType(_sdt) => (),
                AstNode::DimensionList(_dl) => (),
                AstNode::PaddingEntry(_pe) => (),
                AstNode::Range(_r) => (),
                AstNode::ListEntry(_el) => (),
            }
        }
        None
    }
}
