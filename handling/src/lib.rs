use std::{fs::File, io::Write};

pub fn open_or_create(file: &str, content: &str) {
    let mut file = File::options().create(true).open(file).unwrap();
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}

