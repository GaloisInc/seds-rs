//! Validate the SpacePacket Code Generation via Fuzz Testing
use deku::{DekuContainerRead, DekuContainerWrite};
use rand::Rng;
use seds_macro::seds;
use spacepackets::{CcsdsPacket, SpHeader};

// number of fuzz iterations
const NUM_FUZZ: usize = 5000;

#[seds(
    "eds/test/simplified_spacepacket.xml",
    parameters = "eds/test/mission_parameters.json"
)]
struct Dummy; // This will be replaced by the generated module

/// assert header fields are valid between their and our deserialization
fn check_fields_deserial(sp_header: SpHeader, ccsds_buf: [u8; 32]) {
    // make a space packet using the deserialization trait
    let (_, sp) = ccsds::CommandPacket::from_bytes((&ccsds_buf, 0)).unwrap();

    // check the fields
    // VersionId
    assert!(sp_header.ccsds_version() == sp.base.hdr.version_id.value);

    // SecHdrFlags
    assert!(sp_header.sec_header_flag() == (sp.base.hdr.sec_hdr_flags.value == 0));

    // AppId
    assert!(sp_header.apid() == sp.base.hdr.app_id.value);

    // SeqFlag
    assert!(sp_header.sequence_flags() as u8 == sp.base.hdr.seq_flag.value);

    // Sequence
    assert!(sp_header.seq_count() == sp.base.hdr.sequence.value);

    // Length
    assert!(sp_header.data_len() == sp.base.hdr.length.value);
}

/// uniformly sample all possible apids
fn get_random_apid() -> u16 {
    rand::thread_rng().gen_range(0..2048)
}

/// uniformly sample all possible sequence counts
fn get_random_seq() -> u16 {
    rand::thread_rng().gen_range(0..16384)
}

/// uniformly sample all possible data lengths
fn get_random_data_len() -> u16 {
    rand::thread_rng().gen_range(0..65535)
}

/// test a full trip from spacepackets -> ours -> spacepacket
fn check_round_trip() {
    // spacepackets serializer
    let sp_header = SpHeader::tc_unseg(get_random_apid(), get_random_seq(), get_random_data_len())
        .expect("Error creating CCSDS TC header");
    let mut ccsds_buf: [u8; 32] = [0; 32];
    sp_header
        .write_to_be_bytes(&mut ccsds_buf)
        .expect("Writing CCSDS TC header failed");

    // us: deserialize
    let (_, sp) = ccsds::CommandPacket::from_bytes((&ccsds_buf, 0)).unwrap();
    // us: serialize
    let bytes = sp.to_bytes().unwrap();

    // spacepackets: deserialize
    let (sp_header2, _) = SpHeader::from_be_bytes(&bytes).unwrap();

    assert!(sp_header == sp_header2);
}

/// test a full trip from ours -> spacepackets -> ours
fn check_round_trip2() {
    // us: serialize
    let sp = ccsds::CommandPacket {
        base: ccsds::SpacePacket {
            hdr: ccsds::BaseHdr {
                version_id: ccsds::VersionId { value: 0 },
                sec_hdr_flags: ccsds::SecHdrFlags { value: 0 },
                app_id: ccsds::AppId {
                    value: get_random_apid(),
                },
                seq_flag: ccsds::SeqFlag { value: 0 },
                sequence: ccsds::SeqCount {
                    value: get_random_seq(),
                },
                length: ccsds::LengthType {
                    value: get_random_data_len(),
                },
            },
        },
        sec: ccsds::CmdSecHdr {
            command: ccsds::CommandCode { value: 0 },
        },
    };
    let bytes = sp.to_bytes().unwrap();

    // spacepackets: deserialize
    let (sp_header, _) = SpHeader::from_be_bytes(&bytes).unwrap();

    // spacepackets: serialize
    let mut ccsds_buf: [u8; 32] = [0; 32];
    sp_header
        .write_to_be_bytes(&mut ccsds_buf)
        .expect("Writing CCSDS TC header failed");

    // us: deserialize
    let (_, sp2) = ccsds::CommandPacket::from_bytes((&ccsds_buf, 0)).unwrap();

    assert!(sp == sp2);
}

#[test]
fn validate_spacepacket_deserialization() {
    for _ in 0..NUM_FUZZ {
        // make a space packet using the deserialization trait
        let sp_header =
            SpHeader::tc_unseg(get_random_apid(), get_random_seq(), get_random_data_len())
                .expect("Error creating CCSDS TC header");
        let mut ccsds_buf: [u8; 32] = [0; 32];
        sp_header
            .write_to_be_bytes(&mut ccsds_buf)
            .expect("Writing CCSDS TC header failed");

        check_fields_deserial(sp_header, ccsds_buf);
    }
}

#[test]
fn validate_spacepacket_round_trip() {
    for _ in 0..NUM_FUZZ {
        check_round_trip();
        check_round_trip2();
    }
}
