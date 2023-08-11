//! Validate the SpacePacket Code Generation
use deku::DekuContainerRead;
use seds_macro::seds;
use spacepackets::{CcsdsPacket, SpHeader};

#[seds(
    "eds/test/simplified_spacepacket.xml",
    parameters = "eds/test/mission_parameters.json"
)]
struct Dummy; // This will be replaced by the generated module.

#[test]
fn validate_spacepacket() {
    let sp_header = SpHeader::tc_unseg(0x42, 12, 1).expect("Error creating CCSDS TC header");
    let mut ccsds_buf: [u8; 32] = [0; 32];
    sp_header
        .write_to_be_bytes(&mut ccsds_buf)
        .expect("Writing CCSDS TC header failed");

    // make a space packet using the deserialization trait
    let (_, sp) = ccsds::SpacePacket::from_bytes((&ccsds_buf, 0)).unwrap();

    // check the fields
    assert!(sp_header.apid() == sp.hdr.app_id.value);
    assert!(sp_header.seq_count() == sp.hdr.sequence.value);

    // print it out
    println!("Hello, SpacePacket: {:?}", sp);
}
