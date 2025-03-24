use crate::eds_file::AccessMode;

#[test]
pub fn test_parse() {
    assert_eq!(AccessMode::parse("consT"), Some(AccessMode::Constant));
    assert_eq!(AccessMode::parse("ro"), Some(AccessMode::ReadOnly));
    assert_eq!(AccessMode::parse("wo"), Some(AccessMode::WriteOnly));
    assert_eq!(AccessMode::parse("rw"), Some(AccessMode::ReadWrite));
    assert_eq!(
        AccessMode::parse("rww"),
        Some(AccessMode::ReadWritePDOWrite)
    );
    assert_eq!(AccessMode::parse("rwr"), Some(AccessMode::ReadWritePDORead));
    assert_eq!(AccessMode::parse("inv"), None);
    assert_eq!(AccessMode::parse(""), None);
}

#[test]
pub fn test_is_valid() {
    assert_eq!(AccessMode::Constant.is_valid(false), true);
    assert_eq!(AccessMode::ReadOnly.is_valid(false), true);
    assert_eq!(AccessMode::WriteOnly.is_valid(false), true);
    assert_eq!(AccessMode::ReadWrite.is_valid(false), true);
    assert_eq!(AccessMode::ReadWritePDORead.is_valid(false), false);
    assert_eq!(AccessMode::ReadWritePDOWrite.is_valid(false), false);
    assert_eq!(AccessMode::Constant.is_valid(true), true);
    assert_eq!(AccessMode::ReadOnly.is_valid(true), true);
    assert_eq!(AccessMode::WriteOnly.is_valid(true), true);
    assert_eq!(AccessMode::ReadWrite.is_valid(true), false);
    assert_eq!(AccessMode::ReadWritePDORead.is_valid(true), true);
    assert_eq!(AccessMode::ReadWritePDOWrite.is_valid(true), true);
}
