//! SEDS Rust Code Generator
mod format;
use std::collections::HashMap;

pub use format::rustfmt;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use crate::eds::ast::{
    BooleanDataType, ContainerDataType, DataType, EntryElement, Identifier, IntegerDataType,
    NamedEntityType, Package, PackageFile,
};

use heck::{ToPascalCase, ToSnakeCase};
use syn::parse::Error as SynError;

type TypeRefs<'a> = HashMap<String, &'a DataType>;

#[derive(Debug)]
pub enum RustCodegenError {
    InvalidIdentifier(SynError),
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

fn format_snake_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let snake_case = ident_str.to_snake_case();
    syn::parse_str(&snake_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

fn format_pascal_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let pascal_case = ident_str.to_pascal_case();
    syn::parse_str(&pascal_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
}

/// Trait to Implement Field Generation for a Struct
pub trait ToRustField {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError>;
}

pub trait ToRustStruct {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError>;
}

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
            panic!("Multiple Packages not supported")
        }
    }
}

impl ToRustMod for Package {
    fn to_rust_mod(&self, name: Option<&NamedEntityType>) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        // build type references map
        let mut type_refs: HashMap<String, &DataType> = HashMap::new();
        for datatype in self.data_type_set.data_types.iter() {
            let ret = match datatype {
                DataType::IntegerDataType(dt) => {
                    type_refs.insert(dt.name_entity_type.name.0.clone(), datatype)
                }
                DataType::BooleanDataType(dt) => {
                    type_refs.insert(dt.name_entity_type.name.0.clone(), datatype)
                }
                DataType::ContainerDataType(dt) => {
                    type_refs.insert(dt.name_entity_type.name.0.clone(), datatype)
                }
                dt => panic!("Unsupported DataType {:?}", dt),
            };
            if ret.is_some() {
                panic!("Duplicate DataType")
            }
        }

        let mut structs = TokenStream::new();
        for dt in self.data_type_set.data_types.iter() {
            structs.extend(dt.to_rust_struct(None, &type_refs)?);
        }
        Ok(quote!(
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
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_struct(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_struct(name, type_refs),
            DataType::ContainerDataType(dt) => dt.to_rust_struct(name, type_refs),
            dt => panic!("Unsupported DataType {:?}", dt),
        }
    }
}

impl ToRustField for DataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_field(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_field(name, type_refs),
            DataType::ContainerDataType(dt) => dt.to_rust_field(name, type_refs),
            dt => panic!("Unsupported DataType {:?}", dt),
        }
    }
}

impl ToRustField for IntegerDataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        _type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let ty = match self.encoding.size_in_bits {
            0..=8 => quote! { u8 },
            9..=16 => quote! { u16 },
            17..=32 => quote! { u32 },
            33..=64 => quote! { u64 },
            65..=128 => quote! { u128 },
            _ => panic!("Unsupported Integer Size"),
        };
        let sib = format!("{}", self.encoding.size_in_bits);
        let endian = match self.encoding.byte_order {
            crate::eds::ast::ByteOrder::BigEndian => quote! { "big" },
            crate::eds::ast::ByteOrder::LittleEndian => quote! { "little" },
        };
        let description = get_doc_string(name, &self.name_entity_type);
        Ok(quote! {
            #[doc = #description]
            #[deku(bits = #sib, endian = #endian)]
            #sname: #ty,
        })
    }
}

impl ToRustField for BooleanDataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        _type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let ty = match self.encoding.size_in_bits {
            0..=8 => quote! { u8 },
            9..=16 => quote! { u16 },
            17..=32 => quote! { u32 },
            33..=64 => quote! { u64 },
            65..=128 => quote! { u128 },
            _ => panic!("Unsupported Integer Size"),
        };
        let sib = format!("{}", self.encoding.size_in_bits);
        let description = get_doc_string(name, &self.name_entity_type);
        Ok(quote! {
            #[doc = #description]
            #[deku(bits = #sib)]
            #sname: #ty,
        })
    }
}

fn get_traits() -> TokenStream {
    quote! {
        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    }
}

fn get_name(opt_name: Option<&NamedEntityType>, name: &NamedEntityType) -> Ident {
    format_ident!("{}", opt_name.unwrap_or(&name).name.0.to_string())
}

impl ToRustStruct for IntegerDataType {
    fn to_rust_struct(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_pascal_case(&get_name(name, &self.name_entity_type))?;
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
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_pascal_case(&get_name(name, &self.name_entity_type))?;
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

impl ToRustField for ContainerDataType {
    fn to_rust_field(
        &self,
        name: Option<&NamedEntityType>,
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let mut fields = TokenStream::new();
        match &self.base_type {
            Some(bt) => {
                let type_ = *type_refs.get(bt).unwrap();
                fields.extend(type_.to_rust_field(name, type_refs)?);
            }
            None => (),
        }
        match &self.entry_list {
            Some(entries) => {
                for entry in entries.entries.iter() {
                    match entry {
                        EntryElement::Entry(entry) => {
                            let type_ = *type_refs.get(&entry.type_.0).unwrap();
                            let name = &entry.name_entity_type.name.0;
                            let name_entity = NamedEntityType {
                                name: Identifier(name.clone()),
                                short_description: entry.name_entity_type.short_description.clone(),
                                long_description: entry.name_entity_type.long_description.clone(),
                            };
                            let field = type_.to_rust_field(Some(&name_entity), type_refs)?;
                            fields.append_all(field);
                        }
                        _ => println!("Unsupported EntryElement"),
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
        type_refs: &TypeRefs,
    ) -> Result<TokenStream, RustCodegenError> {
        let sname = format_pascal_case(&get_name(name, &self.name_entity_type))?;
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
