//! seds-rs CLI Tool
//! TODO: add to this
use std::fs::{self, OpenOptions};
use std::{fs::File, path::Path};
use std::io::{Write, Read};
use glob::glob;
use seds_rs::expr::ExpressionContext;
use seds_rs::{eds::{raw, ast::PackageFile, resolve::Resolve}, codegen::{rustfmt, codegen_packagefiles}};
use clap::Parser;

pub fn open_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_mission_params() -> ExpressionContext {
    let json = serde_json::json!({
        "CCSDS_SPACEPACKET": {
            "HEADER_TYPE": "BaseHdr",
        },
        "CFE_MISSION": {
            "TELEMETRY_SUBSECONDS_TYPE": "BASE_TYPES/uint16",
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


#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Ethan Lew",
    about = "seds-rs CLI Tool"
)]
struct Args {
    /// XML paths pattern, e.g. eds/**/*xml
    #[clap(required = true)]
    paths: Vec<String>,

    /// Path to JSON mission parameters file
    #[clap(short, long)]
    mission_params: Option<String>,

    /// Output type: stdout, rs, project
    #[clap(short, long, default_value = "stdout")]
    output: String,

    /// Name of the cargo project, if creating a project
    #[clap(short, long)]
    project_name: Option<String>,
}


fn main() {
    let matches = Args::parse(); 
    
    let patterns: Vec<_> = matches.paths;
    let _mission_params_path = matches.mission_params;
    let output_type = matches.output;
    let project_name = matches.project_name;

    // Collect all XML paths
    let mut paths = vec![];
    for pattern in patterns {
        for path in glob(pattern.as_str()).expect("Failed to read glob pattern").flatten() {
            paths.push(path.display().to_string());
        }
    }

    let rpackagefiles: Vec<raw::PackageFile> = paths
        .iter()
        .map(|fp| serde_xml_rs::from_str(&open_file(fp)).unwrap())
        .collect();

    let ectx = get_mission_params();
    let packagefiles: Vec<PackageFile> = rpackagefiles
        .iter()
        .map(|rpf| rpf.resolve(&ectx).unwrap())
        .collect();
    let pfs: Vec<&PackageFile> = packagefiles.iter().collect();
    let code_tokens = codegen_packagefiles(&pfs).unwrap();
    let code = rustfmt(code_tokens).unwrap();

    match output_type.as_str() {
        "stdout" => println!("{}", code),
        "rs" => {
            let mut file = File::create("output.rs").unwrap();
            file.write_all(code.as_bytes()).unwrap();
        }
        "project" => {
            if let Some(name) = project_name {
                create_cargo_project(name.as_str(), &code);
            } else {
                eprintln!("Please provide a project name with -p or --project-name.");
            }
        }
        _ => eprintln!("Invalid output type"),
    }
}

fn create_cargo_project(name: &str, code: &str) {
    // Create a new cargo project and write the code to the main.rs
    std::process::Command::new("cargo")
        .arg("new")
        .arg(name)
        .output()
        .expect("Failed to create new cargo project");

    // Write code to main.rs
    let mut file = File::create(format!("{}/src/lib.rs", name)).expect("Failed to create main.rs");
    file.write_all(code.as_bytes()).expect("Failed to write to main.rs");

    // Read the contents of Cargo.toml
    let cargo_toml_path = format!("{}/Cargo.toml", name);
    let contents = std::fs::read_to_string(&cargo_toml_path)
        .expect("Failed to read Cargo.toml");

    // Add dependencies
    let updated_contents = contents + "\ndeku = \"0.16.0\"";

    // Write updated contents back to Cargo.toml
    let mut cargo_toml_file = OpenOptions::new()
        .write(true)
        .open(&cargo_toml_path)
        .expect("Failed to open Cargo.toml for writing");

    cargo_toml_file.write_all(updated_contents.as_bytes())
        .expect("Failed to write to Cargo.toml");
}
