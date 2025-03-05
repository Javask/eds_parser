use crate::ParseError;
use crate::structured_file::StructuredFileObject;

#[derive(Debug)]
pub enum EDSVersion {
    V3_0,
    V4_0,
    Unsupported,
}

impl EDSVersion {
    pub fn parse(obj: &StructuredFileObject) -> Result<EDSVersion, ParseError> {
        let version_str = obj.get_value("EDSVersion");
        if version_str.is_none() {
            return Ok(EDSVersion::V3_0);
        }
        let parsed = version_str.unwrap().parse::<f32>().or_else(|e| {
            Err(ParseError::ParseFloatError {
                object: "EDSVersion".to_string(),
                section: obj.get_name().to_string(),
                err: e,
            })
        })?;
        if parsed == 3.0 {
            return Ok(EDSVersion::V3_0);
        }
        if parsed == 4.0 {
            return Ok(EDSVersion::V4_0);
        }
        return Ok(EDSVersion::Unsupported);
    }
}
