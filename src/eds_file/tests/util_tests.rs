use crate::{ParseError, eds_file::utils::*, structured_file::StructuredFileObject};

#[test]
fn test_hex_parse() {
    let res = parse_hex_str::<u64, 4>("7FF");
    assert!(res.is_some());
    let res2 = parse_hex_str::<u64, 3>("333");
    assert_eq!(res2.unwrap(), 219);
    assert!(parse_hex_str::<u64, 4>("ü").is_none());
    assert!(parse_hex_str::<u64, 4>("??").is_none());
}

#[test]
fn test_parse_str() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test2".to_string(), "test3".to_string());
    let res = parse_required_str(&obj, "test2");
    assert!(res.is_ok());
    assert_eq!(res.ok().unwrap(), "test3");
    let res2 = parse_required_str(&obj, "test");
    match res2 {
        Err(ParseError::MissingRequiredObject { object, section }) => {
            assert_eq!(object, "test");
            assert_eq!(section, "test_sec");
        }
        _ => panic!(),
    }
}

#[test]
fn test_parse_bool() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test_false".to_string(), "0".to_string());
    obj.get_values_mut()
        .insert("test_true".to_string(), "1".to_string());
    obj.get_values_mut()
        .insert("test_inv1".to_string(), "2".to_string());
    obj.get_values_mut()
        .insert("test_inv2".to_string(), "-1".to_string());
    obj.get_values_mut()
        .insert("test_inv3".to_string(), "a".to_string());
    assert_eq!(parse_required_bool(&obj, "test_false").ok(), Some(false));
    assert_eq!(parse_required_bool(&obj, "test_true").ok(), Some(true));
    assert_eq!(parse_required_bool(&obj, "test_inv1").ok(), None);
    assert_eq!(parse_required_bool(&obj, "test_inv2").ok(), None);
    assert_eq!(parse_required_bool(&obj, "test_inv3").ok(), None);
}

#[test]
fn test_parse_uint() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test_0".to_string(), "0".to_string());
    obj.get_values_mut()
        .insert("test_1".to_string(), "1".to_string());
    obj.get_values_mut()
        .insert("test_256".to_string(), "256".to_string());
    obj.get_values_mut()
        .insert("test_n1".to_string(), "-1".to_string());
    obj.get_values_mut()
        .insert("test_n256".to_string(), "-256".to_string());
    obj.get_values_mut()
        .insert("test_inv".to_string(), "a".to_string());
    obj.get_values_mut()
        .insert("test_hex".to_string(), "0x7FF".to_string());
    obj.get_values_mut()
        .insert("test_oct".to_string(), "0333".to_string());
    obj.get_values_mut()
        .insert("test_hex_inv".to_string(), "0x7FG".to_string());
    obj.get_values_mut()
        .insert("test_oct_inv".to_string(), "033F".to_string());
    assert_eq!(parse_required_uint::<u8>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<u16>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<u32>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<u64>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<i8>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<i16>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<i32>(&obj, "test_0").ok(), Some(0));
    assert_eq!(parse_required_uint::<i64>(&obj, "test_0").ok(), Some(0));

    assert_eq!(parse_required_uint::<u8>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<u16>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<u32>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<u64>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<i8>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<i16>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<i32>(&obj, "test_1").ok(), Some(1));
    assert_eq!(parse_required_uint::<i64>(&obj, "test_1").ok(), Some(1));

    assert_eq!(parse_required_uint::<u8>(&obj, "test_256").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_256").ok(), Some(256));
    assert_eq!(parse_required_uint::<u32>(&obj, "test_256").ok(), Some(256));
    assert_eq!(parse_required_uint::<u64>(&obj, "test_256").ok(), Some(256));
    assert_eq!(parse_required_uint::<i8>(&obj, "test_256").ok(), None);
    assert_eq!(parse_required_uint::<i16>(&obj, "test_256").ok(), Some(256));
    assert_eq!(parse_required_uint::<i32>(&obj, "test_256").ok(), Some(256));
    assert_eq!(parse_required_uint::<i64>(&obj, "test_256").ok(), Some(256));

    assert_eq!(parse_required_uint::<u8>(&obj, "test_n1").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_n1").ok(), None);
    assert_eq!(parse_required_uint::<u32>(&obj, "test_n1").ok(), None);
    assert_eq!(parse_required_uint::<u64>(&obj, "test_n1").ok(), None);
    assert_eq!(parse_required_uint::<i8>(&obj, "test_n1").ok(), Some(-1));
    assert_eq!(parse_required_uint::<i16>(&obj, "test_n1").ok(), Some(-1));
    assert_eq!(parse_required_uint::<i32>(&obj, "test_n1").ok(), Some(-1));
    assert_eq!(parse_required_uint::<i64>(&obj, "test_n1").ok(), Some(-1));

    assert_eq!(parse_required_uint::<u8>(&obj, "test_n256").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_n256").ok(), None);
    assert_eq!(parse_required_uint::<u32>(&obj, "test_n256").ok(), None);
    assert_eq!(parse_required_uint::<u64>(&obj, "test_n256").ok(), None);
    assert_eq!(parse_required_uint::<i8>(&obj, "test_n256").ok(), None);
    assert_eq!(
        parse_required_uint::<i16>(&obj, "test_n256").ok(),
        Some(-256)
    );
    assert_eq!(
        parse_required_uint::<i32>(&obj, "test_n256").ok(),
        Some(-256)
    );
    assert_eq!(
        parse_required_uint::<i64>(&obj, "test_n256").ok(),
        Some(-256)
    );

    assert_eq!(parse_required_uint::<u8>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<u32>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<u64>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<i8>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<i16>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<i32>(&obj, "test_inv").ok(), None);
    assert_eq!(parse_required_uint::<i64>(&obj, "test_inv").ok(), None);

    assert_eq!(parse_required_uint::<u8>(&obj, "test_hex").ok(), None);
    assert_eq!(
        parse_required_uint::<u16>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );
    assert_eq!(
        parse_required_uint::<u32>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );
    assert_eq!(
        parse_required_uint::<u64>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );
    assert_eq!(parse_required_uint::<i8>(&obj, "test_hex").ok(), None);
    assert_eq!(
        parse_required_uint::<i16>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );
    assert_eq!(
        parse_required_uint::<i32>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );
    assert_eq!(
        parse_required_uint::<i64>(&obj, "test_hex").ok(),
        Some(0x7ff)
    );

    assert_eq!(
        parse_required_uint::<u8>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(
        parse_required_uint::<u16>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(
        parse_required_uint::<u32>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(
        parse_required_uint::<u64>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(parse_required_uint::<i8>(&obj, "test_oct").ok(), None);
    assert_eq!(
        parse_required_uint::<i16>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(
        parse_required_uint::<i32>(&obj, "test_oct").ok(),
        Some(0o333)
    );
    assert_eq!(
        parse_required_uint::<i64>(&obj, "test_oct").ok(),
        Some(0o333)
    );

    assert_eq!(parse_required_uint::<u8>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<u32>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<u64>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<i8>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<i16>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<i32>(&obj, "test_hex_inv").ok(), None);
    assert_eq!(parse_required_uint::<i64>(&obj, "test_hex_inv").ok(), None);

    assert_eq!(parse_required_uint::<u8>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<u16>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<u32>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<u64>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<i8>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<i16>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<i32>(&obj, "test_oct_inv").ok(), None);
    assert_eq!(parse_required_uint::<i64>(&obj, "test_oct_inv").ok(), None);
}

#[test]
fn test_parse_date_time_combo() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test_1_time".to_string(), "09:05PM".to_string());
    obj.get_values_mut()
        .insert("test_2_time".to_string(), "09:05PM".to_string());
    obj.get_values_mut()
        .insert("test_1_date".to_string(), "05-16-1999".to_string());
    obj.get_values_mut()
        .insert("test_3_date".to_string(), "05-16-1999".to_string());
    assert!(parse_date_time_combo(&obj, "test_1_date", "test_1_time").is_ok());
    assert!(!parse_date_time_combo(&obj, "test_2_date", "test_2_time").is_ok());
    assert!(!parse_date_time_combo(&obj, "test_3_date", "test_3_time").is_ok());
}
