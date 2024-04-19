
fn main() {
    // Rebuild when OCT package changes.
    println!("cargo:rerun-if-changed=oct.xml");
}