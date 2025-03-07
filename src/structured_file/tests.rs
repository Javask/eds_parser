use crate::ParseError;
use crate::raw_file::RawFile;
use crate::structured_file::StructuredFile;
use crate::tests::utils::*;

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
        ParseError::NoObjectForValue { value } => assert_eq!(value, "Dummy0001"),
        _ => panic!(),
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
        ParseError::InvalidFormatting { line } => assert_eq!(line, "Dummy0001"),
        _ => panic!(),
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
        ParseError::InvalidFormatting { line } => assert_eq!(line, "Dummy0001="),
        _ => panic!(),
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
        ParseError::InvalidFormatting { line } => assert_eq!(line, "=1"),
        _ => panic!(),
    }
}

#[test]
fn test_phase_2_double_section() {
    let data = vec![
        "[DummyUsage]",
        "Dummy0001=aaaa",
        "Dummy0002=2",
        "Dummy0003=--..,.,.-;\"\"",
        "Dummy0004=\"",
        ";Comment",
        "[DummyUsage]",
        "Dummy0005=\"",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::DoubleSectionDefinition { section } => assert_eq!(section, "DummyUsage"),
        _ => panic!(),
    }
}

#[test]
fn test_phase_2_double_section_2() {
    let data = vec![
        "[DummyUsage]",
        "Dummy0001=aaaa",
        "Dummy0002=2",
        "Dummy0003=--..,.,.-;\"\"",
        "Dummy0004=\"",
        ";Comment",
        "[DummyUsage]",
        "Dummy0005=\"",
        "[tmp]",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::DoubleSectionDefinition { section } => assert_eq!(section, "DummyUsage"),
        _ => panic!(),
    }
}

#[test]
fn test_phase_2_double_value() {
    let data = vec![
        "[DummyUsage]",
        "Dummy0001=aaaa",
        "Dummy0001=2",
        "Dummy0003=--..,.,.-;\"\"",
        "Dummy0004=\"",
        ";Comment",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile);
    assert_eq!(sfile.is_err(), true);
    match sfile.err().unwrap() {
        ParseError::DoubleValueDefinition { section, object } => {
            assert_eq!(section, "DummyUsage");
            assert_eq!(object, "Dummy0001");
        }
        _ => panic!(),
    }
}
