use crate::eds_file::eds_time::EDSTime;
use crate::structured_file::StructuredFileObject;

#[test]
fn test_eds_time_parse() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test_1".to_string(), "09:05AM".to_string());
    obj.get_values_mut()
        .insert("test_2".to_string(), "09:05PM".to_string());
    obj.get_values_mut()
        .insert("test_3".to_string(), "12:22AM".to_string());
    obj.get_values_mut()
        .insert("test_4".to_string(), "12:22PM".to_string());
    obj.get_values_mut()
        .insert("test_5".to_string(), "12:22P".to_string());
    obj.get_values_mut()
        .insert("test_6".to_string(), "12:2PM".to_string());
    obj.get_values_mut()
        .insert("test_7".to_string(), "2:22PM".to_string());
    obj.get_values_mut()
        .insert("test_8".to_string(), "".to_string());
    obj.get_values_mut()
        .insert("test_9".to_string(), "13:22PM".to_string());
    obj.get_values_mut()
        .insert("test_10".to_string(), "12:61PM".to_string());

    assert_eq!(
        EDSTime::parse(&obj, "test_1").ok(),
        Some(EDSTime { hour: 9, minute: 5 })
    );
    assert_eq!(
        EDSTime::parse(&obj, "test_2").ok(),
        Some(EDSTime {
            hour: 21,
            minute: 5
        })
    );
    assert_eq!(
        EDSTime::parse(&obj, "test_3").ok(),
        Some(EDSTime {
            hour: 0,
            minute: 22
        })
    );
    assert_eq!(
        EDSTime::parse(&obj, "test_4").ok(),
        Some(EDSTime {
            hour: 12,
            minute: 22
        })
    );
    assert_eq!(EDSTime::parse(&obj, "test_5").ok(), None);
    assert_eq!(EDSTime::parse(&obj, "test_6").ok(), None);
    assert_eq!(EDSTime::parse(&obj, "test_7").ok(), None);
    assert_eq!(EDSTime::parse(&obj, "test_8").ok(), None);
    assert_eq!(EDSTime::parse(&obj, "test_9").ok(), None);
    assert_eq!(EDSTime::parse(&obj, "test_10").ok(), None);
}
