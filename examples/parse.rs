use serde::Deserialize;
use serde_xml_rs::from_str;
use std::env;
use std::fs;

use seds_rs::datasheet::PackageFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <xml_file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let xml_string = fs::read_to_string(file_path).expect("Could not read the XML file");

    match parse_xml::<serde_xml_rs::Error>(&xml_string) {
        Ok(data) => println!("{:#?}", data),
        Err(e) => eprintln!("Failed to parse XML: {:?}", e),
    }
}

fn parse_xml<DeError: std::convert::From<serde_xml_rs::Error>>(
    xml: &str,
) -> Result<PackageFile, DeError> {
    let package_file: PackageFile = from_str(xml)?;
    Ok(package_file)
}
