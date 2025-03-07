use std::fmt::{Debug, Formatter};

pub trait EDSDataType: Debug {}
pub trait EDSData {
    fn format(&self, f: &mut Formatter) -> std::fmt::Result;
}

impl Debug for dyn EDSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl EDSDataType for bool {}
impl EDSDataType for u8 {}
impl EDSDataType for u16 {}
impl EDSDataType for u32 {}
impl EDSDataType for u64 {}
impl EDSDataType for i8 {}
impl EDSDataType for i16 {}
impl EDSDataType for i32 {}
impl EDSDataType for i64 {}
impl EDSDataType for f32 {}
impl EDSDataType for f64 {}
impl EDSDataType for String {}
impl EDSDataType for Vec<u8> {}

#[derive(Debug)]
pub struct EDSDataStruct<E: EDSDataType> {
    val: E,
}
impl<E: EDSDataType> EDSData for EDSDataStruct<E> {
    fn format(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "EDSDataStruct {{{:?}}}", self.val)
    }
}

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

impl<T: EDSDataType + 'static> EDSDataStruct<T> {
    pub fn new(val: T) -> Box<EDSDataStruct<T>> {
        Box::new(EDSDataStruct { val: val })
    }
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
