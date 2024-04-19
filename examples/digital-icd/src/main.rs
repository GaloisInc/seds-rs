//! Crate top level documentation will be here
//!
// Important: note the blank line of documentation on each side of the image lookup table.
// The "image lookup table" can be placed anywhere, but we place it here together with the
// warning if the `doc-images` feature is not enabled.
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("ferris", "rustacean-flat-happy.png")
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![Happy rustacean][ferris]
use seds_macro::seds;

#[seds(
    "../../eds/cFE/modules/core_api/eds/base_types.xml",
    "../../eds/cFE/modules/core_api/eds/ccsds_spacepacket.xml",
    parameters = "../../eds/test/mission_parameters.json"
)]
struct Dummy; // This will be replaced by the generated module

/// The main function doc
fn main() {
    println!("Hello, world!!!");
}
