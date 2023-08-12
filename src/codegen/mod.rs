//! SEDS Rust Code Generator
mod format;
use std::collections::HashMap;

pub use format::rustfmt;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use crate::eds::ast::{
    BooleanDataType, ContainerDataType, DataType, EntryElement, IntegerDataType, NamedEntityType,
    Package, PackageFile,
};

use heck::{ToPascalCase, ToSnakeCase};
use syn::parse::Error as SynError;

/// Type Information (Rust Identifier, SEDS DataType) to Store While Traversing the AST
pub struct RustTypeItem<'a> {
    pub ident: Ident,
    pub data_type: &'a DataType,
}

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

/// RustCodegenError is the error type for the Rust code generator
#[derive(Debug)]
pub enum RustCodegenError {
    InvalidIdentifier(SynError),
    InvalidType(String),
    InvalidBitSize(usize),
    UnsupportedDataType(DataType),
    UnsupportedEntryElement(EntryElement),
    ConflictingDataType(DataType),
    MultiplePackageFiles,
}

/// build the doc string from a NamedEntityType
fn get_doc_string(name: Option<&NamedEntityType>, name_entity_type: &NamedEntityType) -> String {
    let mut description = String::new();
    description.push_str(&format!("{}", &name_entity_type.name.0));
    match name {
        Some(name) => {
            if let Some(short_description) = &name.short_description {
                description.push_str(&format!(" - {}", short_description));
            }
        }
        None => {
            if let Some(short_description) = &name_entity_type.short_description {
                description.push_str(&format!(" - {}", short_description));
            }
        }
    }

    match name {
        Some(name) => {
            if let Some(long_description) = &name.long_description {
                description.push_str(&format!("\n{}", long_description.text));
            }
        }
        None => {
            if let Some(long_description) = &name_entity_type.long_description {
                description.push_str(&format!("\n{}", long_description.text));
            }
        }
    }
    description
}

/// format an identifier to snake_case
fn format_snake_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let snake_case = ident_str.to_snake_case();
    syn::parse_str(&snake_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

/// format an identifier to PascalCase
fn format_pascal_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let pascal_case = ident_str.to_pascal_case();
    syn::parse_str(&pascal_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

/// get the closest, larger unsize type for a given size in bits
fn uint_nearest(size_in_bits: &usize) -> Result<TokenStream, RustCodegenError> {
    match size_in_bits {
        0..=8 => Ok(quote! { u8 }),
        9..=16 => Ok(quote! { u16 }),
        17..=32 => Ok(quote! { u32 }),
        33..=64 => Ok(quote! { u64 }),
        65..=128 => Ok(quote! { u128 }),
        size => Err(RustCodegenError::InvalidBitSize(*size)),
    }
}

/// Trait to Implement Field Generation for a Struct
pub trait ToRustField {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError>;
}

/// Trait to Implement Struct Generation
pub trait ToRustStruct {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError>;
}

/// Trait to Implement Module Generation
pub trait ToRustMod {
    fn to_rust_mod(&self, name: Option<&NamedEntityType>) -> Result<TokenStream, RustCodegenError>;
}

impl ToRustMod for PackageFile {
    fn to_rust_mod(&self, name: Option<&NamedEntityType>) -> Result<TokenStream, RustCodegenError> {
        if self.package.len() == 0 {
            let sname = get_name(name, &NamedEntityType::new("Package"));
            Ok(quote!(
                mod #sname {
                }
            ))
        } else if self.package.len() == 1 {
            self.package[0].to_rust_mod(name)
        } else {
            Err(RustCodegenError::MultiplePackageFiles)
        }
    }
}

impl ToRustMod for Package {
    fn to_rust_mod(&self, name: Option<&NamedEntityType>) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        // build type references map
        // TODO: this is ugly
        let mut type_refs: HashMap<String, RustTypeItem> = HashMap::new();
        for datatype in self.data_type_set.data_types.iter() {
            let ret = match datatype {
                DataType::IntegerDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        ))?,
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::BooleanDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        ))?,
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                DataType::ContainerDataType(dt) => {
                    let item = RustTypeItem {
                        ident: format_pascal_case(&format_ident!(
                            "{}",
                            dt.name_entity_type.name.0
                        ))?,
                        data_type: datatype,
                    };
                    type_refs.insert(dt.name_entity_type.name.0.clone(), item)
                }
                dt => return Err(RustCodegenError::UnsupportedDataType(dt.clone())),
            };
            match ret {
                Some(dt) => {
                    return Err(RustCodegenError::ConflictingDataType(dt.data_type.clone()))
                }
                None => (),
            }
        }

        let rust_types = RustTypeRefs {
            type_refs: type_refs,
        };

        let mut structs = TokenStream::new();
        let description = get_doc_string(name, &self.name_entity_type);
        for dt in self.data_type_set.data_types.iter() {
            structs.extend(dt.to_rust_struct(None, &rust_types)?);
        }
        Ok(quote!(
            #[doc = #description]
            pub mod #sname {
                use deku::{DekuRead, DekuWrite, DekuContainerWrite, DekuUpdate};

                #structs
            }
        ))
    }
}

impl ToRustStruct for DataType {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_struct(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_struct(name, type_refs),
            DataType::ContainerDataType(dt) => dt.to_rust_struct(name, type_refs),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }
}

impl ToRustField for DataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_field(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_field(name, type_refs),
            DataType::ContainerDataType(dt) => dt.to_rust_field(name, type_refs),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }
}

impl ToRustField for IntegerDataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        _type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let ty = uint_nearest(&self.encoding.size_in_bits)?;
        let sib = format!("{}", self.encoding.size_in_bits);
        let endian = match self.encoding.byte_order {
            crate::eds::ast::ByteOrder::BigEndian => quote! { "big" },
            crate::eds::ast::ByteOrder::LittleEndian => quote! { "little" },
        };
        let description = get_doc_string(name, &self.name_entity_type);
        Ok(quote! {
            #[doc = #description]
            #[deku(bits = #sib, endian = #endian)]
            pub #sname: #ty,
        })
    }
}

impl ToRustField for BooleanDataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        _type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let ty = uint_nearest(&self.encoding.size_in_bits)?;
        let sib = format!("{}", self.encoding.size_in_bits);
        let description = get_doc_string(name, &self.name_entity_type);
        Ok(quote! {
            #[doc = #description]
            #[deku(bits = #sib)]
            pub #sname: #ty,
        })
    }
}

/// Get Deku traits for a codegen struct
fn get_traits() -> TokenStream {
    quote! {
        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    }
}

/// Resolve name from an optional NamedEntityType and a NamedEntityType
fn get_name(opt_name: Option<&NamedEntityType>, name: &NamedEntityType) -> Ident {
    format_ident!("{}", opt_name.unwrap_or(&name).name.0.to_string())
}

impl ToRustStruct for IntegerDataType {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = type_refs.lookup_ident(&get_name(name, &self.name_entity_type).to_string())?;
        let field = self.to_rust_field(Some(&NamedEntityType::new("value")), type_refs)?;
        let description = get_doc_string(name, &self.name_entity_type);
        let traits = get_traits();
        Ok(quote! {
            #[doc = #description]
            #traits
            pub struct #sname {
                #field
            }

        })
    }
}

impl ToRustStruct for BooleanDataType {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = type_refs.lookup_ident(&get_name(name, &self.name_entity_type).to_string())?;
        let field = self.to_rust_field(Some(&NamedEntityType::new("value")), type_refs)?;
        let description = get_doc_string(name, &self.name_entity_type);
        let traits = get_traits();
        Ok(quote! {
            #[doc = #description]
            #traits
            pub struct #sname {
                #field
            }

        })
    }
}

/// Get the name of a datatype
fn _get_datatype_name(dt: &DataType) -> Ident {
    let name = match dt {
        DataType::IntegerDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::FloatDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::BooleanDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::ContainerDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::StringDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::EnumeratedDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::ArrayDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::SubRangeDataType(dt) => dt.name_entity_type.name.0.to_string(),
        DataType::NoneDataType => "None".to_string(),
    };
    format_ident!("{}", name)
}

impl ToRustField for ContainerDataType {
    fn to_rust_field(
        &self,
        _: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let mut fields = TokenStream::new();
        match &self.base_type {
            Some(bt) => {
                let type_ = type_refs.lookup_type(&bt)?;
                let tref = _get_datatype_name(&type_);
                let base_field = quote!(
                    pub base: #tref,
                );
                fields.extend(base_field);
            }
            None => (),
        }
        match &self.entry_list {
            Some(entries) => {
                for entry in entries.entries.iter() {
                    match entry {
                        EntryElement::Entry(entry) => {
                            // get type or return invalidtype
                            let type_ = type_refs.lookup_type(&entry.type_.0)?;
                            let name = &format_snake_case(&format_ident!(
                                "{}",
                                entry.name_entity_type.name.0
                            ))?;
                            let tref =
                                type_refs.lookup_ident(&_get_datatype_name(&type_).to_string())?;
                            let description = get_doc_string(
                                Some(&entry.name_entity_type),
                                &entry.name_entity_type,
                            );
                            let field = quote! {
                                #[doc = #description]
                                pub #name: #tref,
                            };
                            fields.append_all(field);
                        }
                        // TODO: duplicate code
                        EntryElement::LengthEntry(entry) => {
                            // get type or return invalidtype
                            let type_ = type_refs.lookup_type(&entry.type_.0)?;
                            let name = &format_snake_case(&format_ident!(
                                "{}",
                                entry.name_entity_type.name.0
                            ))?;
                            let tref =
                                type_refs.lookup_ident(&_get_datatype_name(&type_).to_string())?;
                            let description = get_doc_string(
                                Some(&entry.name_entity_type),
                                &entry.name_entity_type,
                            );
                            let field = quote! {
                                #[doc = #description]
                                pub #name: #tref,
                            };
                            fields.append_all(field);
                        }
                        EntryElement::ErrorControlEntry(_) => (),
                        ee => return Err(RustCodegenError::UnsupportedEntryElement(ee.clone())),
                    }
                }
            }
            None => {}
        }
        Ok(quote! {
            #fields
        })
    }
}

impl ToRustStruct for ContainerDataType {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &RustTypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = type_refs.lookup_ident(&get_name(name, &self.name_entity_type).to_string())?;
        let fields = self.to_rust_field(name, type_refs)?;
        let description = get_doc_string(name, &self.name_entity_type);
        let traits = get_traits();
        Ok(quote! {
            #[doc = #description]
            #traits
            pub struct #sname {
                #fields
            }

        })
    }
}
