use std::fs::File;
use std::fs;

pub fn open_file(s: &str) -> File {
    let file = File::open(s).unwrap();
    file
}

