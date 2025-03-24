use std::fmt::Debug;

pub trait EDSValue: Debug {}
impl EDSValue for bool {}
impl EDSValue for u8 {}
impl EDSValue for u16 {}
impl EDSValue for u32 {}
impl EDSValue for u64 {}
impl EDSValue for i8 {}
impl EDSValue for i16 {}
impl EDSValue for i32 {}
impl EDSValue for i64 {}
impl EDSValue for f32 {}
impl EDSValue for f64 {}
impl EDSValue for String {}
impl EDSValue for Vec<u8> {}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Boolean,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Real32,
    Real64,
    UnicodeString,
    VisibleString,
    OctettString,
    Domain,
}


impl DataType {
    pub fn parse(val: u16) -> Option<DataType> {
        match val {
            0x1 => Some(DataType::Boolean),
            0x2 => Some(DataType::Int8),
            0x3 => Some(DataType::Int16),
            0x4 => Some(DataType::Int32),
            0x5 => Some(DataType::UInt8),
            0x6 => Some(DataType::UInt16),
            0x7 => Some(DataType::UInt32),
            0x8 => Some(DataType::Real32),
            0x9 => Some(DataType::VisibleString),
            0xA => Some(DataType::OctettString),
            0xB => Some(DataType::UnicodeString),
            0xF => Some(DataType::Domain),
            0x11 => Some(DataType::Real64),
            0x15 => Some(DataType::Int64),
            0x1B => Some(DataType::UInt64),
            _ => None,
        }
    }
}
