//! Context for Code Generation
use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::format::{format_pascal_case, format_snake_case};
use crate::eds::ast::{DataType, Identifier, NamedEntityType, Package, PackageFile};

use super::RustCodegenError;

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
            name,
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

    /// Get an identifier from locals or another module in the global namespace
    pub fn get_qualified_ident(&self, path: &str) -> Result<TokenStream, RustCodegenError> {
        let segments = path.split('/').collect::<Vec<_>>();

        match segments.len() {
            1 => {
                // Single identifier
                let ident = format_ident!("{}", segments[0]); // Create an Ident from the string
                let pascal_ident = format_pascal_case(&ident)?;
                Ok(quote! { #pascal_ident })
            }
            2 => {
                // Module and identifier
                let module_ident = format_ident!("{}", segments[0]);
                let snake_module = format_snake_case(&module_ident)?;
                let ident = format_ident!("{}", segments[1]);
                let pascal_ident = format_pascal_case(&ident)?;
                Ok(quote! { #snake_module::#pascal_ident })
            }
            _ => Err(RustCodegenError::InvalidType(path.into())),
        }
    }
}

/// Namespace struct that supports nested namespaces
#[derive(Debug, Clone)]
pub struct Namespace<'a> {
    /// optional namespace identifier
    pub name: Option<Identifier>,
    /// map from ast item strings to rust type items
    pub type_refs: HashMap<String, RustTypeItem<'a>>,
    /// optional children namespace
    pub children: Option<Vec<Namespace<'a>>>,
}

/// implement conversion from relevant ast concepts
impl<'a> TryFrom<Vec<&'a PackageFile>> for Namespace<'a> {
    type Error = RustCodegenError;

    fn try_from(value: Vec<&'a PackageFile>) -> Result<Self, RustCodegenError> {
        let package_vecs: Vec<&Vec<Package>> = value.iter().map(|p| &p.package).collect();
        let packages: Vec<&Package> = package_vecs.iter().copied().flatten().collect();
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
                DataType::NoneDataType => None,
                DataType::ArrayDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
                DataType::SubRangeDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
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
                DataType::EnumeratedDataType(dt) => {
                    let (k, v) = prepare_item(&dt.name_entity_type.name.0, datatype)?;
                    type_refs.insert(k, v)
                }
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
            type_refs,
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

/// Type Information (Rust Identifier, SEDS DataType) to Store While Traversing the AST
#[derive(Debug, Clone)]
pub struct RustTypeItem<'a> {
    /// Rust Identifier
    pub ident: Ident,
    /// DataType from SEDS Ast
    pub data_type: &'a DataType,
}

#[derive(Debug, Clone)]
/// Rust Type Refs Available to Traverser of the AST
pub struct RustTypeRefs<'a> {
    type_refs: HashMap<String, RustTypeItem<'a>>,
}

impl<'a> RustTypeRefs<'a> {
    /// Lookup a type by name
    pub fn lookup_type(&self, name: &String) -> Result<&'a DataType, RustCodegenError> {
        let lu = self.type_refs.get(name);
        match lu {
            Some(t) => Ok(t.data_type),
            None => Err(RustCodegenError::InvalidType(name.clone())),
        }
    }

    /// Lookup an identifier by name
    pub fn lookup_ident(&self, name: &String) -> Result<&Ident, RustCodegenError> {
        let lu = self.type_refs.get(name);
        match lu {
            Some(t) => Ok(&t.ident),
            None => Err(RustCodegenError::InvalidType(name.clone())),
        }
    }
}
