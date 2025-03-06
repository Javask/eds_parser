use crate::{ParseError, structured_file::StructuredFileObject};

use super::Address;
#[derive(Debug)]
pub struct EDSObject {}

impl EDSObject {
    pub fn parse(_obj: &StructuredFileObject, _addr: &Address) -> Result<EDSObject, ParseError> {
        Ok(EDSObject {})
    }
}
