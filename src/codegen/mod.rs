//! SEDS Rust Code Generator
pub mod context;
pub mod convert;
pub mod convert_rust;
pub mod dependency;
pub mod format;

pub use convert::*;
pub use format::rustfmt;

use crate::eds::ast::{DataType, EntryElement};
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::parse::Error as SynError;

/// Type Information (Rust Identifier, SEDS DataType) to Store While Traversing the AST
#[derive(Debug, Clone)]
pub struct RustTypeItem<'a> {
    pub ident: Ident,
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
