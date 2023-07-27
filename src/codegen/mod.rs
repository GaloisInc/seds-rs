//! SEDS Rust Code Generator
mod format;
pub use format::rustfmt;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::eds::ast::{DataType, Identifier, IntegerDataType, NamedEntityType};

#[derive(Debug)]
pub enum RustCodegenError {}

/// Trait to Implement Field Generation for a Deku Struct
pub trait ToDekuField {
    fn to_deku_field(&self) -> Result<TokenStream, RustCodegenError>;
}

pub trait ToDekuStruct {
    fn to_deku_struct(&self) -> Result<TokenStream, RustCodegenError>;
}

impl ToDekuField for DataType {
    fn to_deku_field(&self) -> Result<TokenStream, RustCodegenError> {
        match self {
            DataType::IntegerDataType(dt) => dt.to_deku_field(),
            _ => panic!("Unsupported DataType"),
        }
    }
}

impl ToDekuField for IntegerDataType {
    fn to_deku_field(&self) -> Result<TokenStream, RustCodegenError> {
        let name = format_ident!("{}", &self.name_entity_type.name.0);
        let ty = match self.encoding.size_in_bits {
            8 => quote! { u8 },
            16 => quote! { u16 },
            32 => quote! { u32 },
            64 => quote! { u64 },
            _ => panic!("Unsupported Integer Size"),
        };
        let sib = format!("{}", self.encoding.size_in_bits);
        let endian = match self.encoding.byte_order {
            crate::eds::ast::ByteOrder::BigEndian => quote! { "big" },
            crate::eds::ast::ByteOrder::LittleEndian => quote! { "little" },
        };
        Ok(quote! {
            #[deku(bits = #sib, endian = #endian)]
            #name: #ty,
        })
    }
}

fn get_traits() -> TokenStream {
    quote! {
        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    }
}

fn get_doc_string(name_entity_type: &NamedEntityType) -> String {
    let mut description = String::new();
    description.push_str(&format!("{}", &name_entity_type.name.0));
    if let Some(long) = &name_entity_type.long_description {
        description.push_str(&format!("\n{}", long.text));
    } else if let Some(short) = &name_entity_type.short_description {
        description.push_str(&format!("\n{}", short));
    };
    description
}

impl ToDekuStruct for IntegerDataType {
    fn to_deku_struct(&self) -> Result<TokenStream, RustCodegenError> {
        let name = format_ident!("{}", &self.name_entity_type.name.0);
        let mut val_self = self.clone();
        val_self.name_entity_type.name = Identifier("value".to_string());
        let field = val_self.to_deku_field()?;
        let description = get_doc_string(&self.name_entity_type);
        let traits = get_traits();
        Ok(quote! {
            #[doc = #description]
            #traits
            struct #name {
                #field
            }
        })
    }
}
