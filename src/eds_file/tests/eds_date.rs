use crate::eds_file::eds_date::EDSDate;
use crate::structured_file::StructuredFileObject;

#[test]
fn test_eds_date_parse() {
    let mut obj = StructuredFileObject::new("test_sec".to_string());
    obj.get_values_mut()
        .insert("test_1".to_string(), "01-01-0001".to_string());
    obj.get_values_mut()
        .insert("test_2".to_string(), "12-30-2018".to_string());
    obj.get_values_mut()
        .insert("test_3".to_string(), "12-3-2018".to_string());
    obj.get_values_mut()
        .insert("test_4".to_string(), "12-30-201".to_string());
    obj.get_values_mut()
        .insert("test_5".to_string(), "1-30-2018".to_string());
    obj.get_values_mut()
        .insert("test_6".to_string(), "13-30-2018".to_string());
    obj.get_values_mut()
        .insert("test_7".to_string(), "12-32-2018".to_string());
    obj.get_values_mut()
        .insert("test_8".to_string(), "".to_string());
    assert_eq!(
        EDSDate::parse(&obj, "test_1").ok(),
        Some(EDSDate {
            year: 1,
            month: 1,
            day: 1
        })
    );
    assert_eq!(
        EDSDate::parse(&obj, "test_2").ok(),
        Some(EDSDate {
            year: 2018,
            month: 12,
            day: 30
        })
    );
    assert_eq!(EDSDate::parse(&obj, "test_3").ok(), None);
    assert_eq!(EDSDate::parse(&obj, "test_4").ok(), None);
    assert_eq!(EDSDate::parse(&obj, "test_5").ok(), None);
    assert_eq!(EDSDate::parse(&obj, "test_6").ok(), None);
    assert_eq!(EDSDate::parse(&obj, "test_7").ok(), None);
    assert_eq!(EDSDate::parse(&obj, "test_8").ok(), None);
}
