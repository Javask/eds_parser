mod eds_device_info;
mod eds_file_info;

mod eds_date;
mod eds_time;
mod eds_version;

mod utils;

pub use eds_device_info::EDSDeviceInfo;
pub use eds_file_info::EDSFileInfo;
pub use eds_version::EDSVersion;

use super::structured_file::StructuredFile;
use crate::ParseError;

pub struct EDSFile {
    pub file_info: EDSFileInfo,
    pub device_info: EDSDeviceInfo,
}

impl EDSFile {
    pub(crate) fn parse(sfile: StructuredFile) -> Result<EDSFile, ParseError> {
        let file_info_obj =
            sfile
                .get_object("FileInfo")
                .ok_or(ParseError::MissingRequiredSection {
                    section: "FileInfo".to_string(),
                })?;
        let file_info = EDSFileInfo::parse(file_info_obj)?;
        let device_info_obj =
            sfile
                .get_object("DeviceInfo")
                .ok_or(ParseError::MissingRequiredSection {
                    section: "DeviceInfo".to_string(),
                })?;
        let device_info = EDSDeviceInfo::parse(device_info_obj)?;


        Ok(EDSFile {
            file_info: file_info,
            device_info: device_info,
        })
    }
}
