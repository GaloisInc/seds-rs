
fn main() {
    // Rebuild when OCT package changes.
    println!("cargo:rerun-if-changed=../../eds/cFE/modules/core_api/eds/base_types.xml");
}