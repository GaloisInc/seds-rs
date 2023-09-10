//! SEDS Rust Code Generator
pub mod context;
pub mod convert;
pub mod dependency;
pub mod format;

pub use convert::*;
pub use format::rustfmt;
use proc_macro2::TokenStream;

use crate::eds::ast::{DataType, EntryElement, PackageFile};
use syn::parse::Error as SynError;

use self::context::{CodegenContext, Namespace};

/// RustCodegenError is the error type for the Rust code generator
#[derive(Debug)]
pub enum RustCodegenError {
    /// Syn Crate Identifier Error
    InvalidIdentifier(SynError),
    /// String cannot resolve to a type somewhere
    InvalidType(String),
    /// usize value cannot map to a bit size
    InvalidBitSize(usize),
    /// DataType isn't supported (yet)
    UnsupportedDataType(Box<DataType>),
    /// EntryElement isn't supported (yet)
    UnsupportedEntryElement(Box<EntryElement>),
    /// DataType conflicts with another one
    ConflictingDataType(Box<DataType>),
}

/// CodeGen function to convert packagefiles to a tokenstream
pub fn codegen_packagefiles(pfs: &[&PackageFile]) -> Result<TokenStream, RustCodegenError> {
    let mut generated_code = proc_macro2::TokenStream::new();
    let namespace = Namespace::try_from(pfs.to_owned())?;
    for pf in pfs.iter() {
        for pkg in pf.package.iter() {
            let locals = Namespace::try_from(pkg)?;
            let ctx = CodegenContext {
                name: None,
                locals: &locals,
                namespace: &namespace,
            };
            let code = pkg.to_rust_mod(&ctx)?;
            generated_code.extend(code);
        }
    }

    Ok(generated_code)
}
