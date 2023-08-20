use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use crate::eds::ast::{
    BooleanDataType, ContainerDataType, DataType, EntryElement, FloatDataType, IntegerDataType,
    NamedEntityType, Package, PackageFile, QualifiedName, StringDataType,
};

use super::{
    context::CodegenContext,
    dependency::{AstNode, QualifiedNameIter},
    format::{format_pascal_case, format_snake_case},
    RustCodegenError,
};

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

/// Get all depencies mentioned in a package as imports tokenstream
fn get_package_imports(pkg: &Package) -> Result<TokenStream, RustCodegenError> {
    // collect the necessary imports
    let mut imports = TokenStream::new();
    let qni = QualifiedNameIter::new(AstNode::Package(pkg));
    let mut qnames: Vec<&QualifiedName> = qni.into_iter().collect();
    qnames.dedup();

    for path in qnames.iter() {
        let segments = path.0.split('/').collect::<Vec<_>>();

        match segments.len() {
            1 => (),
            2 => {
                // Module and identifier
                let module_ident = format_ident!("{}", segments[0]);
                let snake_module = format_snake_case(&module_ident)?;
                let ident = format_ident!("{}", segments[1]);
                let pascal_ident = format_pascal_case(&ident)?;
                imports.extend(quote!(
                    use #snake_module::#pascal_ident;
                ));
            }
            _ => return Err(RustCodegenError::InvalidType(path.0.to_string())),
        }
    }

    Ok(imports)
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

        let imports = get_package_imports(&self)?;

        Ok(quote!(
            #[doc = #description]
            pub mod #sname {
                use deku::{DekuRead, DekuWrite, DekuContainerWrite, DekuUpdate};
                #imports

                #structs
            }
        ))
    }
}

impl ToRustTokens for DataType {
    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_struct(&ctx),
            DataType::FloatDataType(dt) => dt.to_rust_struct(&ctx),
            DataType::BooleanDataType(dt) => dt.to_rust_struct(&ctx),
            DataType::ContainerDataType(dt) => dt.to_rust_struct(&ctx),
            DataType::StringDataType(dt) => dt.to_rust_struct(&ctx),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }

    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_rust_field(&ctx),
            DataType::FloatDataType(dt) => dt.to_rust_field(&ctx),
            DataType::BooleanDataType(dt) => dt.to_rust_field(&ctx),
            DataType::ContainerDataType(dt) => dt.to_rust_field(&ctx),
            DataType::StringDataType(dt) => dt.to_rust_field(&ctx),
            dt => Err(RustCodegenError::UnsupportedDataType(dt.clone())),
        }
    }
}

impl ToRustTokens for StringDataType {
    fn to_rust_field(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = format_snake_case(&get_name(name, &self.name_entity_type))?;
        let description = get_doc_string(name, &self.name_entity_type);
        let length_ident = format_ident!("{}_dlen", sname);
        let update_str = format!("self.{}.len()", sname);
        let count_str = format!("{}_dlen", sname);
        Ok(quote! {
            #[doc = #description]
            #[deku(update = #update_str)]
            #length_ident: u8,
            #[deku(count = #count_str)]
            #sname: Vec<char>,
        })
    }

    fn to_rust_struct(&self, ctx: &CodegenContext) -> Result<TokenStream, RustCodegenError> {
        let name = ctx.name;
        let sname = &ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident;
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

impl ToRustTokens for FloatDataType {
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
        let sname = &ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident;
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
        let sname = &ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident;
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
        let sname = &ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident;
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
                            let tref = ctx.get_qualified_ident(&entry.type_.0)?;
                            let name = &format_snake_case(&format_ident!(
                                "{}",
                                entry.name_entity_type.name.0
                            ))?;
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
                            let tref =
                                ctx.get_qualified_ident(&get_datatype_name(&type_).to_string())?;
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
        let sname = &ctx
            .lookup_ident(&get_name(name, &self.name_entity_type).to_string())?
            .ident;
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
