use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(PartialEq, Clone, Eq, Copy)]
pub struct Address {
    pub index: u16,
    pub subindex: u8,
}

impl Address {
    pub fn new(index: u16, subindex: u8) -> Address {
        Address {
            index: index,
            subindex: subindex,
        }
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Address {{index: 0x{:X}, subindex:0x{:X}}}",
            self.index, self.subindex
        )
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address {{0x{:X}.{:X}}}", self.index, self.subindex)
    }
}

impl Hash for Address {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.index);
        state.write_u8(self.subindex);
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp1 = ((self.index as u32) << 8) | (self.subindex as u32);
        let cmp2 = ((other.index as u32) << 8) | (other.subindex as u32);
        cmp1.partial_cmp(&cmp2)
    }
}
