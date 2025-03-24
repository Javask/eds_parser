use crate::raw_file::RawFile;
use crate::tests::utils::*;

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
