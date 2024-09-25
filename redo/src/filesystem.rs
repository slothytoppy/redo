use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn read<P: AsRef<std::path::Path>>(file_name: P) -> String {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(file_name.as_ref())
        .expect("file should exist and should be readable");
    let mut buffer = String::default();
    let _ = file.read_to_string(&mut buffer);
    buffer
}

pub fn write<P: AsRef<std::path::Path>>(file_name: P, contents: String) -> bool {
    match OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .truncate(false)
        .open(file_name.as_ref())
    {
        Ok(mut file) => {
            let _ = file.write(contents.as_bytes());
            true
        }
        Err(..) => false,
    }
}
