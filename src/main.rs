mod datasheet;

use datasheet::PackageFile;
use serde_xml_rs::from_str;

fn main() {
    //let s = "<your xml string here>";
    //let package: PackageFile = from_str(s).unwrap();

    // open the xml file as a string
    let s = std::fs::read_to_string("test.xml").unwrap();
    let package: PackageFile = serde_xml_rs::from_str(&s).unwrap();
    println!("{:#?}", package);
}
