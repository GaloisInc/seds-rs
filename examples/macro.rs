//! Example of Macro Code Generation for SEDS
use deku::DekuContainerRead;
use seds_macro::seds;

#[seds(
    "eds/test/simplified_spacepacket.xml",
    parameters = "eds/test/mission_parameters.json"
)]
struct Dummy; // This will be replaced by the generated module.

fn main() {
    // make a space packet using the deserialization trait
    let sp = ccsds::SpacePacket::from_bytes((&[0xDA; 6], 0)).unwrap();

    // print it out
    println!("Hello, SpacePacket: {:?}", sp);
}
