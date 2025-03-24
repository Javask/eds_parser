use std::{
    fs::File,
    io::{Seek, Write},
};
use tempfile::tempfile;

pub fn make_tmp_file(data: String) -> File {
    let mut file = tempfile().expect("Failed to create temporary file!");
    file.write(data.as_bytes())
        .expect("Failed to write to temporary file!");
    file.seek(std::io::SeekFrom::Start(0))
        .expect("Failed to rewind file!");
    return file;
}

pub fn make_string(lines: &Vec<&str>) -> String {
    let mut out = String::new();
    for line in lines {
        if out.is_empty() {
            out = line.to_string();
        } else {
            out = out + "\n" + line;
        }
    }
    out
}
