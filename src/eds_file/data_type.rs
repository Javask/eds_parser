use std::fmt::Debug;
use std::marker::Send;

pub trait EDSValue: Debug + Send {
    fn clone_box(&self) -> Box<dyn EDSValue>;
}

impl EDSValue for bool {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for u8 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for u16 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for u32 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for u64 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for i8 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for i16 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for i32 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for i64 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for f32 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for f64 {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for String {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}
impl EDSValue for Vec<u8> {
    fn clone_box(&self) -> Box<dyn EDSValue> {
        Box::new(self.clone())
    }
}

pub fn clone_eds_value(x: &Option<Box<dyn EDSValue>>) -> Option<Box<dyn EDSValue>> {
    match x {
        None => None,
        Some(val) => Some(val.clone_box()),
    }
}

#[derive(Debug, PartialEq, Clone)]
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
