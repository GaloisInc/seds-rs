use std::{fs, io::Read, path::Path};

use seds_rs::expr::ExpressionContext;

/// unsafe way to load file into string
/// (unsafe because it assumes the file exists and it will fail the test if not)
#[allow(dead_code)]
pub fn open_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// default mission parameters for testing
#[allow(dead_code)]
pub fn get_mission_params() -> ExpressionContext {
    let json = serde_json::json!({
        "CCSDS_SPACEPACKET": {
            "HEADER_TYPE": "BaseHdr",
        },
        "CFE_MISSION": {
            "TELEMETRY_SUBSECONDS_TYPE": "BASE_TYPES/uint32",
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
