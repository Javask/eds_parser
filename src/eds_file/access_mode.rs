use std::fmt::Display;
#[derive(Debug, PartialEq)]
pub enum AccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    ReadWritePDOWrite,
    ReadWritePDORead,
    Constant,
}

impl Display for AccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AccessMode::Constant => write!(f, "const"),
            AccessMode::ReadOnly => write!(f, "ro"),
            AccessMode::ReadWrite => write!(f, "rw"),
            AccessMode::ReadWritePDORead => write!(f, "rwr"),
            AccessMode::ReadWritePDOWrite => write!(f, "rww"),
            AccessMode::WriteOnly => write!(f, "wo"),
        }
    }
}

impl AccessMode {
    pub fn parse(val: &str) -> Option<AccessMode> {
        let lower = val.to_lowercase();
        match lower.as_str() {
            "ro" => Some(AccessMode::ReadOnly),
            "wo" => Some(AccessMode::WriteOnly),
            "rw" => Some(AccessMode::ReadWrite),
            "rwr" => Some(AccessMode::ReadWritePDORead),
            "rww" => Some(AccessMode::ReadWritePDOWrite),
            "const" => Some(AccessMode::Constant),
            _ => None,
        }
    }
    pub fn is_valid(&self, pdo_mappable: bool) -> bool {
        if pdo_mappable {
            match &self {
                AccessMode::ReadWrite => false,
                _ => true,
            }
        } else {
            match &self {
                AccessMode::ReadWritePDORead => false,
                AccessMode::ReadWritePDOWrite => false,
                _ => true,
            }
        }
    }
}
