//! Collect Dependencies for Codegen
use crate::eds::ast::*;

/// put all relevant ast items in an enum so we iterate over them
#[derive(Debug)]
pub enum AstNode<'a> {
    PackageFile(&'a PackageFile),
    Package(&'a Package),
    NamedEntityType(&'a NamedEntityType),
    DataTypeSet(&'a DataTypeSet),
    DataType(&'a DataType),
    IntegerDataType(&'a IntegerDataType),
    FloatDataType(&'a FloatDataType),
    StringDataType(&'a StringDataType),
    ContainerDataType(&'a ContainerDataType),
    BooleanDataType(&'a BooleanDataType),
    EntryList(&'a EntryList),
    EntryElement(&'a EntryElement),
    ErrorControlEntry(&'a ErrorControlEntry),
    Entry(&'a Entry),
    LengthEntry(&'a LengthEntry),
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
                    self.stack.push(AstNode::NamedEntityType(&pkg.name_entity_type));
                    self.stack.push(AstNode::DataTypeSet(&pkg.data_type_set));
                }
                AstNode::DataTypeSet(dts) => {
                    for dt in dts.data_types.iter().rev() {
                        self.stack.push(AstNode::DataType(dt));
                    }
                }
                AstNode::DataType(datatype) => {
                    match datatype {
                        DataType::IntegerDataType(dt) => self.stack.push(AstNode::IntegerDataType(dt)),
                        DataType::ContainerDataType(dt) => self.stack.push(AstNode::ContainerDataType(dt)),
                        DataType::FloatDataType(dt) => self.stack.push(AstNode::FloatDataType(dt)),
                        DataType::StringDataType(dt) => self.stack.push(AstNode::StringDataType(dt)),
                        DataType::BooleanDataType(dt) => self.stack.push(AstNode::BooleanDataType(dt)),
                        other => panic!("{:?} not supported", other),
                    }
                }
                AstNode::ContainerDataType(cdt) => {
                    self.stack.push(AstNode::NamedEntityType(&cdt.name_entity_type));
                    match &cdt.entry_list {
                        Some(el) => self.stack.push(AstNode::EntryList(el)),
                        None => ()
                    }
                }
                AstNode::EntryList(el) => {
                    for entry in el.entries.iter().rev() {
                        self.stack.push(AstNode::EntryElement(entry));
                    }
                }
                AstNode::EntryElement(ee) => {
                    match ee {
                        EntryElement::Entry(e) => self.stack.push(AstNode::Entry(e)),
                        EntryElement::LengthEntry(e) => self.stack.push(AstNode::LengthEntry(e)),
                        EntryElement::ErrorControlEntry(e) => self.stack.push(AstNode::ErrorControlEntry(e)),
                        other => panic!("{:?} not supported", other)
                    }
                }
                AstNode::Entry(e) => {
                    self.stack.push(AstNode::NamedEntityType(&e.name_entity_type));
                    return Some(&e.type_);
                }
                AstNode::LengthEntry(le) => {
                    self.stack.push(AstNode::NamedEntityType(&le.name_entity_type));
                    return Some(&le.type_);
                }
                AstNode::ErrorControlEntry(ece) => {
                    self.stack.push(AstNode::NamedEntityType(&ece.name_entity_type));
                    return Some(&ece.type_);
                }
                AstNode::IntegerDataType(_idt) => (),
                AstNode::BooleanDataType(_bdt) => (),
                AstNode::NamedEntityType(_net) => (),
                other => panic!("{:?} not supported", other)
            }
        }
        None
    }
}