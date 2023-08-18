//! Context for Code Generation
use std::collections::HashMap;

use heck::ToPascalCase;
use proc_macro2::Ident;
use quote::format_ident;

use crate::eds::ast::{NamedEntityType, PackageFile, Package, Identifier, DataType};

use super::{RustTypeRefs, RustTypeItem, RustCodegenError};

use std::convert::From;

/// format an identifier to PascalCase
fn format_pascal_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let pascal_case = ident_str.to_pascal_case();
    syn::parse_str(&pascal_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

/// CodegenContext houses all the necessary information for 
/// code generation trait
pub struct CodegenContext<'a> {
    /// NamedEntityType of the AST item
    pub name: Option<&'a NamedEntityType>,
    /// Type References
    pub type_refs: &'a RustTypeRefs<'a>,
}

/// Namespace struct that supports nested namespaces
pub struct Namespace<'a> {
    pub name: Option<Identifier>,
    pub type_refs: HashMap<String, RustTypeItem<'a>>,
    pub children: Option<Vec<Namespace<'a>>>
}

impl<'a> From<Vec<&'a PackageFile>> for Namespace<'a> {
    fn from(value: Vec<&'a PackageFile>) -> Self {
        let package_vecs: Vec<&Vec<Package>> = value.iter().map(|p| &p.package).collect();
        let packages: Vec<&Package> = package_vecs.iter().map(|v| *v).flatten().collect();
        Namespace {
            name: None,
            type_refs: HashMap::new(),
            children: Some(packages.into_iter().map(|p| Namespace::from(p)).collect()),
        }
    }
}

impl<'a> From<&'a Package> for Namespace<'a> {
    /// TODO: this conversion is ugly but now it works
    fn from(value: &'a Package) -> Self {
        let mut type_refs: HashMap<String, RustTypeItem> = HashMap::new();
        for datatype in value.data_type_set.data_types.iter() {
            let ret = match datatype {
                DataType::IntegerDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        )).unwrap(),
                        data_type: datatype, 
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::FloatDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        )).unwrap(),
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::StringDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        )).unwrap(),
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::BooleanDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        )).unwrap(),
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::ContainerDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        )).unwrap(),
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                dt => panic!("unsupported datatype {:?}", dt) 
            };
            match ret {
                Some(_) => {
                    panic!() 
                }
                None => (),
            }
        }

       Namespace {
           name: Some(value.name_entity_type.name.clone()),
           type_refs: type_refs,
           children: None,
       }
    }
}