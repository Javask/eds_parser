mod eds_device_info;
mod eds_file_info;

mod address;
mod object_type;
mod access_mode;
mod data_type;

mod eds_date;
mod eds_object;
mod eds_time;
mod eds_version;

mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use address::Address;
pub use access_mode::AccessMode;
pub use object_type::ObjectType;

pub use eds_device_info::EDSDeviceInfo;
pub use eds_file_info::EDSFileInfo;
pub use eds_object::*;
pub use eds_version::EDSVersion;
use utils::parse_required_uint;

use super::structured_file::StructuredFile;
use crate::ParseError;

#[derive(Debug)]
pub struct EDSFile {
    pub file_info: EDSFileInfo,
    pub device_info: EDSDeviceInfo,
    pub mandatory_object: HashMap<Address, EDSObject>,
    pub optional_object: HashMap<Address, EDSObject>,
    pub manufacturer_object: HashMap<Address, EDSObject>,
}

impl EDSFile {
    fn parse_object_list(
        sfile: &StructuredFile,
        name: &str,
    ) -> Result<HashMap<Address, EDSObject>, ParseError> {
        let obj = sfile
            .get_object(name)
            .ok_or(ParseError::MissingRequiredSection {
                section: name.to_string(),
            })?;
        let supported_obj_count: u16 = parse_required_uint(obj, "SupportedObjects")?;
        let mut map = HashMap::new();
        for i in 0..supported_obj_count {
            let index = parse_required_uint::<u16>(obj, &(i + 1).to_string())?;
            let addr = Address::new(index, 0);
            let obj = EDSObject::parse(sfile, &addr)?;
            map.insert(addr, obj);
        }
        Ok(map)
    }

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
        let mandatory_objects = Self::parse_object_list(&sfile, "MandatoryObjects")?;
        let optional_objects = Self::parse_object_list(&sfile, "OptionalObjects")?;
        let manufacturer_objects = Self::parse_object_list(&sfile, "ManufacturerObjects")?;

        Ok(EDSFile {
            file_info: file_info,
            device_info: device_info,
            mandatory_object: mandatory_objects,
            optional_object: optional_objects,
            manufacturer_object: manufacturer_objects,
        })
    }
}
