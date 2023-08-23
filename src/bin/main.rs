//! seds-rs CLI Tool
//! TODO: add to this
use clap::Parser;
use glob::glob;
use seds_rs::expr::ExpressionContext;
use seds_rs::{
    codegen::{codegen_packagefiles, rustfmt},
    eds::{ast::PackageFile, raw, resolve::Resolve},
};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::{fs::File, path::Path};

/// open a file with the correct io Result return
pub fn open_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// load the necessary JSON parameters
/// TODO: support EDS Design Parameters
pub fn load_mission_params_from_file(filepath: &str) -> io::Result<ExpressionContext> {
    let contents = std::fs::read_to_string(filepath)?;
    let json: serde_json::Value = serde_json::from_str(&contents)?;
    ExpressionContext::from_json(&json).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to create ExpressionContext from JSON",
        )
    })
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Ethan Lew", about = "seds-rs CLI Tool")]
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

fn main() -> io::Result<()> {
    let matches = Args::parse();

    let patterns: Vec<_> = matches.paths;
    let output_type = matches.output;
    let project_name = matches.project_name;

    // Collect all XML paths
    let mut paths = vec![];
    for pattern in patterns {
        for path in glob(pattern.as_str())
            .expect("Failed to read glob pattern")
            .flatten()
        {
            paths.push(path.display().to_string());
        }
    }

    let rpackagefiles: Vec<raw::PackageFile> = paths
        .iter()
        .map(|fp| serde_xml_rs::from_str(&open_file(fp)?))
        .collect::<Result<_, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Parse error: {:?}", e)))?;

    let ectx = if let Some(ref params_path) = matches.mission_params {
        load_mission_params_from_file(params_path)?
    } else {
        ExpressionContext::new()
    };

    let packagefiles: Vec<PackageFile> = rpackagefiles
        .iter()
        .map(|rpf| rpf.resolve(&ectx))
        .collect::<Result<_, _>>()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Expression resolution error: {:?}", e),
            )
        })?;
    let pfs: Vec<&PackageFile> = packagefiles.iter().collect();
    let code_tokens = codegen_packagefiles(&pfs)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Codegen error: {:?}", e)))?;
    let code = rustfmt(code_tokens)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Rustfmt error: {:?}", e)))?;

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

    Ok(())
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
    file.write_all(code.as_bytes())
        .expect("Failed to write to main.rs");

    // Read the contents of Cargo.toml
    let cargo_toml_path = format!("{}/Cargo.toml", name);
    let contents = std::fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");

    // Add dependencies
    let updated_contents = contents + "\ndeku = \"0.16.0\"";

    // Write updated contents back to Cargo.toml
    let mut cargo_toml_file = OpenOptions::new()
        .write(true)
        .open(&cargo_toml_path)
        .expect("Failed to open Cargo.toml for writing");

    cargo_toml_file
        .write_all(updated_contents.as_bytes())
        .expect("Failed to write to Cargo.toml");
}
