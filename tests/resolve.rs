mod common;

use common::open_file;
use seds_rs::eds::raw::PackageFile;
use seds_rs::eds::resolve::Resolve;
use seds_rs::expr::ExpressionContext;

#[test]
fn test_resolve_spacepacket() {
    let contents = open_file("./eds/cFE/modules/core_api/eds/ccsds_spacepacket.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    let json = serde_json::json!({
        "CCSDS_SPACEPACKET": {
            "HEADER_TYPE": "<HEADER_TYPE>",
        },
        "CFE_MISSION": {
            "TELEMETRY_SUBSECONDS_TYPE": "<SUBSECONDS_TYPE>",
        },
    });
    let ectx = ExpressionContext::from_json(&json).unwrap();
    // println!("{:#?}", package.resolve(&ectx));
    assert!(package.resolve(&ectx).is_ok());
}
