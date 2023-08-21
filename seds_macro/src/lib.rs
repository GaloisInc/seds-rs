//! SEDS Macro Crate
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use syn::{parse_macro_input, AttributeArgs, NestedMeta, Lit, Meta};

use seds_rs::eds::raw;
use seds_rs::eds::ast;
use seds_rs::expr::ExpressionContext;
use seds_rs::eds::resolve::Resolve;
use seds_rs::codegen::ToRustMod;
use seds_rs::codegen::context::{Namespace, CodegenContext};

#[proc_macro_attribute]
pub fn seds(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let mut xml_files: Vec<String> = Vec::new();
    let mut parameters: Option<String> = None;

    for arg in attr_args {
        match arg {
            NestedMeta::Lit(Lit::Str(s)) => {
                xml_files.push(s.value().into());
            }
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("parameters") => {
                if let Lit::Str(s) = nv.lit {
                    parameters = Some(s.value().into());
                }
            }
            _ => {}
        }
    }

    let mut generated_code = proc_macro2::TokenStream::new();
    let mut package_files = Vec::<ast::PackageFile>::new();
    for xml_file in xml_files.iter() {
        // Check if file exists
        let path = std::path::Path::new(xml_file);
        if !path.exists() {
            panic!("File {} does not exist", xml_file);
        }

        // Read the file content
        let file_content = std::fs::read_to_string(xml_file)
            .expect(&format!("Failed to read the file: {}", xml_file));

        // Parse the XML content
        let rpf: raw::PackageFile = serde_xml_rs::from_str(&file_content)
            .expect(&format!("Failed to parse the file: {}", xml_file));

        let ectx = if let Some(params_file) = &parameters {
            let params_content = std::fs::read_to_string(params_file)
                .expect(&format!("Failed to read the parameters file: {}", params_file));
            
            let json = serde_json::from_str(&params_content.as_str())
                .expect(&format!("Failed to parse the parameters file: {}", params_file));

            ExpressionContext::from_json(&json)
                .expect(format!("Failed to parse the parameters file {}", params_file).as_str())
        } else {
            let json = serde_json::json!({});
            ExpressionContext::from_json(&json).expect("Failed to parse the default parameters")
        };

        let pf_result = rpf.resolve(&ectx);
        if let Err(e) = pf_result {
            panic!("Failed to resolve the package file: {:?}", e);
        }
        let pf = pf_result.unwrap();
        package_files.push(pf);
    } 

    for xml_file in xml_files.iter() {
        // Check if file exists
        let path = std::path::Path::new(xml_file);
        if !path.exists() {
            panic!("File {} does not exist", xml_file);
        }

        // Read the file content
        let file_content = std::fs::read_to_string(xml_file)
            .expect(&format!("Failed to read the file: {}", xml_file));

        // Parse the XML content
        let rpf: raw::PackageFile = serde_xml_rs::from_str(&file_content)
            .expect(&format!("Failed to parse the file: {}", xml_file));

        let ectx = if let Some(params_file) = &parameters {
            let params_content = std::fs::read_to_string(params_file)
                .expect(&format!("Failed to read the parameters file: {}", params_file));
            
            let json = serde_json::from_str(&params_content.as_str())
                .expect(&format!("Failed to parse the parameters file: {}", params_file));

            ExpressionContext::from_json(&json)
                .expect(format!("Failed to parse the parameters file {}", params_file).as_str())
        } else {
            let json = serde_json::json!({});
            ExpressionContext::from_json(&json).expect("Failed to parse the default parameters")
        };

        let pf_result = rpf.resolve(&ectx);
        if let Err(e) = pf_result {
            panic!("Failed to resolve the package file: {:?}", e);
        }
        let pf = pf_result.unwrap(); 

        // Generate Rust code
        let pfs: Vec<&ast::PackageFile> = package_files.iter().collect();
        let namespace = Namespace::try_from(pfs).unwrap();
        for pkg in pf.package.iter() {
            let locals = Namespace::try_from(pkg).unwrap();
            let ctx = CodegenContext {
                name: None,
                locals: &locals,
                namespace: &namespace,
            };
            let code_result = pf.to_rust_mod(&ctx);
            if let Err(e) = code_result {
                panic!("Failed to generate Rust code: {:?}", e);
            }
            generated_code.extend(code_result.unwrap());
        }
    }
    
    generated_code.into()
}
