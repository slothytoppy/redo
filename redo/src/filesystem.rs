use std::{fs::OpenOptions, io::Read};

pub fn read<P: AsRef<std::path::Path>>(file_name: P) -> String {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(file_name.as_ref())
        .expect("cant open the file");
    let mut buffer = String::default();
    let _ = file.read_to_string(&mut buffer);
    buffer
}

pub fn write<P: AsRef<std::path::Path>>(file_name: P, contents: String) -> bool {
    if !file_name.as_ref().exists() {
        return false;
    }
    let _ = std::fs::write(file_name, contents);
    true
}
