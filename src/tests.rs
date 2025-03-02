use std::{
    fs::File,
    io::{Seek, Write},
};
use tempfile::tempfile;

use super::*;

fn make_tmp_file(data: String) -> File {
    let mut file = tempfile().expect("Failed to create temporary file!");
    file.write(data.as_bytes())
        .expect("Failed to write to temporary file!");
    file.seek(std::io::SeekFrom::Start(0))
        .expect("Failed to rewind file!");
    return file;
}

fn make_string(lines: &Vec<&str>) -> String {
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

#[test]
fn test_phase_1_file_read() {
    let data = vec!["Hello World!", "Second Line!"];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let lines = rfile.get_lines();
    if data.len() != lines.len() {
        panic!("Read different amount of lines!");
    }
    println!("Read {} lines:", lines.len());
    for i in 0..data.len() {
        println!("{}", lines[i]);
        if data[i] != lines[i] {
            panic!("Mismatched lines!");
        }
    }
}

#[test]
fn test_phase_2_success() {
    let data = vec![
        "[DummyUsage]",
        "Dummy0001=aaaa",
        "Dummy0002=2",
        "Dummy0003=--..,.,.-;\"\"",
        "Dummy0004=\"",
        ";Comment",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let parsed_objs = sfile.get_objects();
    assert_eq!(parsed_objs.len(), 1);
    let dummy_obj = &parsed_objs[0];
    assert_eq!(dummy_obj.get_name(), "DummyUsage");
    let dummy_vals = dummy_obj.get_values();
    assert_eq!(dummy_vals.len(), 4);
    assert_eq!(dummy_vals["Dummy0001"], "aaaa");
    assert_eq!(dummy_vals["Dummy0002"], "2");
    assert_eq!(dummy_vals["Dummy0003"], "--..,.,.-;\"\"");
    assert_eq!(dummy_vals["Dummy0004"], "\"");
}

#[test]
fn test_phase_2_no_object() {
    let data = vec!["Dummy0001=aaaa", "[DummyUsage]"];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::IOError(_) => panic!(),
        ParseError::InvalidFormatting { line: _ } => panic!(),
        ParseError::NoObjectForValue { value } => assert_eq!(value, "Dummy0001"),
        ParseError::DoubleValueDefinition {
            object: _,
            value: _,
        } => panic!(),
    }
}

#[test]
fn test_phase_2_invalid_lines_1() {
    let data = vec!["[DummyUsage]", "Dummy0001"];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::IOError(_) => panic!(),
        ParseError::InvalidFormatting { line } => assert_eq!(line, "Dummy0001"),
        ParseError::NoObjectForValue { value: _ } => panic!(),
        ParseError::DoubleValueDefinition {
            object: _,
            value: _,
        } => panic!(),
    }
}

#[test]
fn test_phase_2_invalid_lines_2() {
    let data = vec!["[DummyUsage]", "Dummy0001="];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::IOError(_) => panic!(),
        ParseError::InvalidFormatting { line } => assert_eq!(line, "Dummy0001="),
        ParseError::NoObjectForValue { value: _ } => panic!(),
        ParseError::DoubleValueDefinition {
            object: _,
            value: _,
        } => panic!(),
    }
}


#[test]
fn test_phase_2_invalid_lines_3() {
    let data = vec!["[DummyUsage]", "=1"];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::IOError(_) => panic!(),
        ParseError::InvalidFormatting { line } => assert_eq!(line, "=1"),
        ParseError::NoObjectForValue { value: _ } => panic!(),
        ParseError::DoubleValueDefinition {
            object: _,
            value: _,
        } => panic!(),
    }
}


#[test]
fn test_integration(){
    let rfile = RawFile::new("res/test.eds").expect("Failed to open file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse valid EDS!");
    println!("Objs: {}",sfile.get_objects().len());
}