use chrono::{DateTime, Utc};

use super::EDSVersion;
use crate::ParseError;
use crate::eds_file::utils::*;
use crate::structured_file::StructuredFileObject;

#[derive(Debug, Clone)]
pub struct EDSFileInfo {
    pub file_name: String,
    pub file_version: u8,
    pub file_revision: u8,
    pub eds_version: EDSVersion,
    pub description: String,
    pub creation: DateTime<Utc>,
    pub created_by: String,
    pub modification: DateTime<Utc>,
    pub modified_by: String,
}

impl EDSFileInfo {
    pub(crate) fn parse(obj: &StructuredFileObject) -> Result<EDSFileInfo, ParseError> {
        let file_name = parse_required_str(obj, "FileName")?;
        let file_version = parse_required_uint(obj, "FileVersion")?;
        let file_revision = parse_required_uint(obj, "FileRevision")?;
        let eds_version = EDSVersion::parse(obj)?;
        let description = parse_required_str(obj, "Description")?;
        let creation = parse_date_time_combo(obj, "CreationDate", "CreationTime")?;
        let created_by = parse_required_str(obj, "CreatedBy")?;
        let modification = parse_date_time_combo(obj, "ModificationDate", "ModificationTime")?;
        let modified_by = parse_required_str(obj, "ModifiedBy")?;

        Ok(EDSFileInfo {
            file_name: file_name.clone(),
            file_version: file_version,
            file_revision: file_revision,
            eds_version: eds_version,
            description: description.clone(),
            creation: creation,
            created_by: created_by.clone(),
            modification: modification,
            modified_by: modified_by.clone(),
        })
    }
}
