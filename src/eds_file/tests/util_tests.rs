use crate::eds_file::utils::*;

#[test]
fn test_hex_parse() {
    let res = parse_hex_str::<u64, 4>("7FF");
    assert!(res.is_some());
    let res2 = parse_hex_str::<u64, 3>("333");
    assert_eq!(res2.unwrap(), 219);
    assert!(parse_hex_str::<u64, 4>("ü").is_none());
    assert!(parse_hex_str::<u64, 4>("??").is_none());
}
