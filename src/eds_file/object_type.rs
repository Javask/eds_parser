use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Variable,
    Array,
    Record,
    Deftype,
    Defstruct,
    Domain,
    Null,
}

impl ObjectType {
    pub fn parse(val: u8) -> Option<ObjectType> {
        match val {
            0x00 => Some(ObjectType::Null),
            0x02 => Some(ObjectType::Domain),
            0x05 => Some(ObjectType::Deftype),
            0x06 => Some(ObjectType::Defstruct),
            0x07 => Some(ObjectType::Variable),
            0x08 => Some(ObjectType::Array),
            0x09 => Some(ObjectType::Record),
            _ => None,
        }
    }
}

impl Display for ObjectType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            ObjectType::Null => write!(f,"Null"),
            ObjectType::Domain => write!(f,"Domain"),
            ObjectType::Deftype => write!(f,"Deftype"),
            ObjectType::Defstruct => write!(f,"Defstruct"),
            ObjectType::Variable => write!(f,"Variable"),
            ObjectType::Array => write!(f,"Array"),
            ObjectType::Record => write!(f,"Record"),
        }
    }
}