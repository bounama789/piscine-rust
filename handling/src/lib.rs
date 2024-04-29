use std::{fs::File, io::Write};

pub fn open_or_create(file: &str, content: &str) {
    let mut file = File::create(file).expect("Failed to create file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}

