//! SEDS Macro Crate
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, NestedMeta, Lit, Meta};

use seds_rs::eds::raw;
use seds_rs::expr::ExpressionContext;
use seds_rs::eds::resolve::Resolve;
use seds_rs::codegen::ToRustMod;

#[proc_macro_attribute]
pub fn seds(attr: TokenStream, _item: TokenStream) -> TokenStream {
    // Parse the input streams into syntax trees
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let mut xml_file = String::new();
    let mut parameters: Option<String> = None;

    for arg in attr_args {
        match arg {
            NestedMeta::Lit(Lit::Str(s)) => {
                xml_file = s.value().into();
            }
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("parameters") => {
                if let Lit::Str(s) = nv.lit {
                    parameters = Some(s.value().into());
                }
            }
            _ => {}
        }
    }

    // Check if file exists
    let path = std::path::Path::new(&xml_file);
    if !path.exists() {
        panic!("File {} does not exist", xml_file);
    }

    // Read the file content
    let file_content = std::fs::read_to_string(&xml_file)
        .expect(&format!("Failed to read the file: {}", xml_file));

    // Parse the XML content
    let rpf: raw::PackageFile = serde_xml_rs::from_str(&file_content)
        .expect(&format!("Failed to parse the file: {}", xml_file));

    let ectx = if let Some(params_file) = parameters {
        // Load parameters from the provided JSON file
        let params_content = std::fs::read_to_string(&params_file)
            .expect(&format!("Failed to read the parameters file: {}", params_file));
        
        // Parse the JSON content into your expected type here
        let json = serde_json::from_str(&params_content.as_str())
            .expect(&format!("Failed to parse the parameters file: {}", params_file));

        ExpressionContext::from_json(&json)
            .expect(format!("Failed to parse the parameters file {}", params_file).as_str())
    } else {
        let json = serde_json::json!({});
        ExpressionContext::from_json(&json).expect("Failed to parse the default parameters")
    };
   
    // Run the resolver
    let pf_result = rpf.resolve(&ectx);
    if let Err(e) = pf_result {
        panic!("Failed to resolve the package file: {:?}", e);
    }
    let pf = pf_result.unwrap(); 

    // Generate Rust code
    let generated_code_result = pf.to_rust_mod(None);
    if let Err(e) = generated_code_result {
        panic!("Failed to generate Rust code: {:?}", e);
    }
    // Format the generated code
    let generated_code = generated_code_result.unwrap();

    // Convert the generated code into a TokenStream and return
    generated_code.into()
}
