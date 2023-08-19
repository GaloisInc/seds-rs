//! Context for Code Generation
use std::collections::HashMap;

use heck::ToPascalCase;
use proc_macro2::Ident;
use quote::format_ident;

use crate::eds::ast::{DataType, Identifier, NamedEntityType, Package, PackageFile};

use super::{RustCodegenError, RustTypeItem};

/// format an identifier to PascalCase
fn format_pascal_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let pascal_case = ident_str.to_pascal_case();
    syn::parse_str(&pascal_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

/// CodegenContext houses all the necessary information for
/// code generation trait
pub struct CodegenContext<'a> {
    /// NamedEntityType of the current AST item
    pub name: Option<&'a NamedEntityType>,
    /// Namespace for Local Type References
    pub locals: &'a Namespace<'a>,
    /// Namespace for Global Type References
    pub namespace: &'a Namespace<'a>,
}

impl<'a> CodegenContext<'a> {
    /// rename a context and keep all other references the same
    pub fn change_name(&self, name: Option<&'a NamedEntityType>) -> Self {
        CodegenContext {
            name: name,
            locals: self.locals,
            namespace: self.namespace,
        }
    }

    /// Looks up an identifier by its path.
    pub fn lookup_ident(&self, path: &str) -> Result<&RustTypeItem<'a>, RustCodegenError> {
        let res = if path.contains('/') {
            // Lookup in the namespace
            self.namespace.find_type_item(path)
        } else {
            self.locals.find_type_item(path)
        };
        match res {
            Some(item) => Ok(item),
            None => Err(RustCodegenError::InvalidType(path.to_string())),
        }
    }
}

/// Namespace struct that supports nested namespaces
#[derive(Debug, Clone)]
pub struct Namespace<'a> {
    pub name: Option<Identifier>,
    pub type_refs: HashMap<String, RustTypeItem<'a>>,
    pub children: Option<Vec<Namespace<'a>>>,
}

/// implement conversion from relevant ast concepts
impl<'a> TryFrom<Vec<&'a PackageFile>> for Namespace<'a> {
    type Error = RustCodegenError;

    fn try_from(value: Vec<&'a PackageFile>) -> Result<Self, RustCodegenError> {
        let package_vecs: Vec<&Vec<Package>> = value.iter().map(|p| &p.package).collect();
        let packages: Vec<&Package> = package_vecs.iter().map(|v| *v).flatten().collect();
        Ok(Namespace {
            name: None,
            type_refs: HashMap::new(),
            children: Some(
                packages
                    .into_iter()
                    .filter_map(|p| Namespace::try_from(p).ok())
                    .collect(),
            ),
        })
    }
}

/// helper method to build a rust item from a name and datatype
fn prepare_item<'a>(
    sname: &String,
    datatype: &'a DataType,
) -> Result<(String, RustTypeItem<'a>), RustCodegenError> {
    let item = RustTypeItem {
        ident: format_pascal_case(&format_ident!("{}", sname))?,
        data_type: datatype,
    };

    Ok((sname.clone(), item))
}

/// implement conversion from relevant ast concepts
impl<'a> TryFrom<&'a Package> for Namespace<'a> {
    type Error = RustCodegenError;

    /// TODO: this conversion is ugly but now it works
    fn try_from(value: &'a Package) -> Result<Self, RustCodegenError> {
        let mut type_refs: HashMap<String, RustTypeItem> = HashMap::new();
        for datatype in value.data_type_set.data_types.iter() {
            let ret = match datatype {
                DataType::IntegerDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                DataType::FloatDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                DataType::StringDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                DataType::BooleanDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                DataType::ContainerDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                dt => return Err(RustCodegenError::UnsupportedDataType(dt.clone())),
            };
            match ret {
                Some(item) => {
                    return Err(RustCodegenError::ConflictingDataType(
                        item.data_type.clone(),
                    ))
                }
                None => (),
            }
        }

        Ok(Namespace {
            name: Some(value.name_entity_type.name.clone()),
            type_refs: type_refs,
            children: None,
        })
    }
}

impl<'a> Namespace<'a> {
    /// lookup RustTypeItem reference by path string
    pub fn find_type_item(&self, path: &str) -> Option<&RustTypeItem<'a>> {
        let mut path_segments = path.split('/').collect::<Vec<_>>();

        if path_segments.is_empty() {
            return None;
        }

        let current_segment = path_segments.remove(0);

        // If this is the last segment, look up in type_refs
        if path_segments.is_empty() {
            return self.type_refs.get(current_segment);
        }

        // Otherwise, recurse into children
        if let Some(children) = &self.children {
            for child in children {
                if let Some(ref name) = child.name {
                    if name.0 == current_segment {
                        return child.find_type_item(&path_segments.join("/"));
                    }
                }
            }
        }

        None
    }
}
