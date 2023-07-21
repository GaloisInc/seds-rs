mod common;

use common::open_file;
use seds_rs::eds::raw::PackageFile;
use seds_rs::eds::resolve::resolve_package_file;

#[test]
fn test_3_2_1() {
    let contents = open_file("eds/test/test_resolved.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    println!("{:#?}", resolve_package_file(&package));
}
