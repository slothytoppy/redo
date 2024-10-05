use std::fs::OpenOptions;
use std::io::{Read, Write};

use tracing;

pub fn read<P: AsRef<std::path::Path>>(file_name: P) -> Option<String> {
    let mut buffer = String::default();
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(file_name.as_ref());

    if let Ok(mut file) = file {
        if file.read_to_string(&mut buffer).is_err() {
            return None;
        }
        Some(buffer)
    } else {
        None
    }
}

pub fn write<P: AsRef<std::path::Path>>(file_name: P, contents: String) -> bool {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name.as_ref())
    {
        Ok(mut file) => {
            let _ = file.write(contents.as_bytes());
            true
        }
        Err(e) => {
            tracing::info!("{:?}", e);
            false
        }
    }
}
