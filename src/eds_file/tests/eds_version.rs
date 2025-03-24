use crate::{eds_file::EDSVersion, structured_file::StructuredFileObject};

#[test]
pub fn test_parse_3() {
    let mut obj = StructuredFileObject::new("test".to_string());
    obj.get_values_mut()
        .insert("EDSVersion".to_string(), "3.0".to_string());
    let v = EDSVersion::parse(&obj).expect("Failed to parse v3.0 string");
    assert_eq!(v, EDSVersion::V3_0);

    let obj2 = StructuredFileObject::new("test".to_string());
    let v = EDSVersion::parse(&obj2).expect("Failed to parse no string");
    assert_eq!(v, EDSVersion::V3_0);
}

#[test]
pub fn test_parse_4() {
    let mut obj = StructuredFileObject::new("test".to_string());
    obj.get_values_mut()
        .insert("EDSVersion".to_string(), "4.0".to_string());
    let v = EDSVersion::parse(&obj).expect("Failed to parse v4.0 string");
    assert_eq!(v, EDSVersion::V4_0);
}

#[test]
pub fn test_parse_unsupported() {
    let mut obj = StructuredFileObject::new("test".to_string());
    obj.get_values_mut()
        .insert("EDSVersion".to_string(), "4.1".to_string());
    let v = EDSVersion::parse(&obj).expect("Failed to parse unsupported string");
    assert_eq!(v, EDSVersion::Unsupported);
}

#[test]
pub fn test_parse_invalid() {
    let mut obj = StructuredFileObject::new("test".to_string());
    obj.get_values_mut()
        .insert("EDSVersion".to_string(), "aaa".to_string());
    let v = EDSVersion::parse(&obj);
    assert!(v.is_err());
}
