//! SEDS Macro Crate
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, NestedMeta, Lit};

use seds_rs::eds::raw;
use seds_rs::expr::ExpressionContext;
use seds_rs::eds::resolve::Resolve;
use seds_rs::codegen::ToRustMod;

/// TODO: default mission parameters for testing
fn get_mission_params() -> ExpressionContext {
    let json = serde_json::json!({
        "CCSDS_SPACEPACKET": {
            "HEADER_TYPE": "BaseHdr",
        },
        "CFE_MISSION": {
            "TELEMETRY_SUBSECONDS_TYPE": "<SUBSECONDS_TYPE>",
            "SIGNED_INTEGER_ENCODING": "signMagnitude",
            "DATA_BYTE_ORDER": "littleEndian",
            "MEM_REFERENCE_SIZE_BITS": "2",
            "ES_CDS_MAX_FULL_NAME_LEN": "2",
            "EVS_MAX_MESSAGE_LENGTH": "2",
            "MAX_CPU_ADDRESS_SIZE": "1024",
            "ES_MAX_APPLICATIONS": "2",
            "FS_HDR_DESC_MAX_LEN": "2",
            "MAX_PATH_LEN": "2",
            "MAX_API_LEN": "2",
            "SB_MAX_PIPES": "2",
            "ES_PERF_MAX_IDS": "2",
            "ES_POOL_MAX_BUCKETS": "2",
            "TBL_MAX_FULL_NAME_LEN": "2",
        },
        "CFE_SB": {
            "MSGID_BIT_SIZE": "2",
            "SUB_ENTRIES_PER_PKT": "2",
        },
        "CFE_FS": {
            "HDR_DESC_MAX_LEN": "2",
        },
    });
    ExpressionContext::from_json(&json).unwrap()
}

#[proc_macro_attribute]
pub fn seds(attr: TokenStream, _item: TokenStream) -> TokenStream {
    // Parse the input streams into syntax trees
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    // Extract the XML file name from the attribute
    let mut xml_file = String::new();
    for arg in attr_args {
        if let NestedMeta::Lit(Lit::Str(s)) = arg {
            xml_file = s.value().into();
        }
    }
    
    // check if file exists
    let path = std::path::Path::new(&xml_file);
    if !path.exists() {
        panic!("File {} does not exist", xml_file);
    }
    // Read the file content
    let file_content = std::fs::read_to_string(&xml_file)
        .expect(&format!("Failed to read the file: {}", xml_file));

    // Parse the XML content
    let rpf: raw::PackageFile = serde_xml_rs::from_str(&file_content).unwrap();

    // TODO: allow use to pass in mission parameters
    let ectx = get_mission_params();
   
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
    let generated_code = generated_code_result.unwrap();

    // Convert the generated code into a TokenStream and return
    generated_code.into()
}
