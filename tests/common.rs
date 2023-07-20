use std::{fs, io::Read, path::Path};

/// unsafe way to load file into string
/// (unsafe because it assumes the file exists and it will fail the test if not)
pub fn open_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
