use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use crate::eds::ast::{
    BooleanDataType, ContainerDataType, DataType, EntryElement, IntegerDataType, NamedEntityType,
    Package, PackageFile,
};

use super::{context::CodegenContext, RustCodegenError};

/// Trait for DataTypes
pub trait ToRustTokens {
    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError>;

    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError>;
}

/// Trait for module generators
pub trait ToRustMod {
    fn to_rust_mod(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError>;
}

/// Resolve name from an optional NamedEntityType and a NamedEntityType
fn get_name(opt_name: Option<&NamedEntityType>, name: &NamedEntityType) -> Ident {
    format_ident!("{}", opt_name.unwrap_or(&name).name.0.to_string())
}

/// format an identifier to snake_case
fn format_snake_case(ident: &Ident) -> Result<Ident, RustCodegenError> {
    let ident_str = ident.to_string();
    let snake_case = ident_str.to_snake_case();
    syn::parse_str(&snake_case).map_err(|e| RustCodegenError::InvalidIdentifier(e))
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

/// Get Deku traits for a codegen struct
fn get_traits() -> TokenStream {
    quote! {
        #[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
    }
}

/// Get the name of a datatype
/// TODO: should be a public method?
fn get_datatype_name(dt: &DataType) -> Ident {
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

impl ToRustMod for PackageFile {
    fn to_rust_mod(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        if self.package.len() == 0 {
            let sname = get_name(name, &NamedEntityType::new("Package"));
            Ok(quote!(
                mod #sname {
                }
            ))
        } else if self.package.len() == 1 {
            let nctx = ctx.change_name(name);
            self.package[0].to_rust_mod(&nctx)
        } else {
            Err(RustCodegenError::MultiplePackageFiles)
        }
    }
}

impl ToRustMod for Package {
    fn to_rust_mod(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let name = ctx.name;
        let mut structs = TokenStream::new();
        let description = get_doc_string(name, &self.name_entity_type);
        for dt in self.data_type_set.data_types.iter() {
            let nctx = ctx.change_name(None);
            structs.extend(dt.to_rust_struct(&nctx)?);
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

impl ToRustTokens for DataType {
    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_struct(&ctx),
            //DataType::FloatDataType(dt) => dt.to_rust_struct(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_struct(&ctx),
            DataType::ContainerDataType(dt) => dt.to_rust_struct(&ctx),
            //DataType::StringDataType(dt) => dt.to_rust_struct(name, type_refs),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }

    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_field(&ctx),
            //DataType::FloatDataType(dt) => dt.to_rust_field(name, type_refs),
            DataType::BooleanDataType(dt) => dt.to_rust_field(&ctx),
            DataType::ContainerDataType(dt) => dt.to_rust_field(&ctx),
            //DataType::StringDataType(dt) => dt.to_rust_field(name, type_refs),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }
}

impl ToRustTokens for IntegerDataType {
    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
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

    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident
            .to_string();
        let field_name = NamedEntityType::new("value");
        let nctx = ctx.change_name(Some(&field_name));
        let field = self.to_rust_field(&nctx)?;
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

impl ToRustTokens for BooleanDataType {
    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
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

    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident
            .to_string();
        let field_name = NamedEntityType::new("value");
        let nctx = ctx.change_name(Some(&field_name));
        let field = self.to_rust_field(&nctx)?;
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

impl ToRustTokens for ContainerDataType {
    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let mut fields = TokenStream::new();
        match &self.base_type {
            Some(bt) => {
                let type_ = ctx.lookup_ident(&bt)?.data_type;
                let tref = get_datatype_name(&type_);
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
                            let type_ = ctx.lookup_ident(&entry.type_.0)?.data_type;
                            let name = &format_snake_case(&format_ident!(
                                "{}",
                                entry.name_entity_type.name.0
                            ))?;
                            let tref = &ctx
                                .lookup_ident(&get_datatype_name(&type_).to_string())?
                                .ident;
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
                            let type_ = ctx.lookup_ident(&entry.type_.0)?.data_type;
                            let name = &format_snake_case(&format_ident!(
                                "{}",
                                entry.name_entity_type.name.0
                            ))?;
                            let tref = &ctx
                                .lookup_ident(&get_datatype_name(&type_).to_string())?
                                .ident;
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

    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident
            .to_string();
        let nctx = ctx.change_name(name);
        let fields = self.to_rust_field(&nctx)?;
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
