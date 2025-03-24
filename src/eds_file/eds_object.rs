use std::fmt::Debug;

use crate::{
    ParseError,
    structured_file::{StructuredFile, StructuredFileObject},
};

use super::{
    Address,
    access_mode::AccessMode,
    clone_eds_value,
    data_type::{DataType, EDSValue},
    object_type::ObjectType,
    utils::{
        parse_required_bool, parse_required_double, parse_required_float, parse_required_hex_data,
        parse_required_str, parse_required_uint,
    },
};

#[derive(Debug)]
pub enum EDSObject {
    EDSNull {
        address: Address,
        name: String,
        object_type: ObjectType,
    },
    EDSVariable {
        address: Address,
        name: String,
        object_type: ObjectType,
        data_type: DataType,
        access_mode: AccessMode,
        default: Option<Box<dyn EDSValue>>,
        pdo_mappable: bool,
        low_limit: Option<Box<dyn EDSValue>>,
        high_limit: Option<Box<dyn EDSValue>>,
        refuse_write_on_download: bool,
        refuse_read_on_scan: bool,
    },
    EDSArray {
        address: Address,
        name: String,
        object_type: ObjectType,
        sub_number: u8,
        entries: Vec<EDSObject>,
        refuse_write_on_download: bool,
        refuse_read_on_scan: bool,
    },
    CompactEDSArray {
        address: Address,
        name: String,
        object_type: ObjectType,
        data_type: DataType,
        access_mode: AccessMode,
        default: Option<Box<dyn EDSValue>>,
        pdo_mappable: bool,
        low_limit: Option<Box<dyn EDSValue>>,
        high_limit: Option<Box<dyn EDSValue>>,
        refuse_write_on_download: bool,
        refuse_read_on_scan: bool,
    },
    EDSDomain {
        address: Address,
        name: String,
        object_type: ObjectType,
        data_type: DataType,
        access_mode: AccessMode,
        default: Option<Box<dyn EDSValue>>,
        refuse_write_on_download: bool,
        refuse_read_on_scan: bool,
    },
}

impl Clone for EDSObject {
    fn clone(&self) -> Self {
        match &self {
            EDSObject::CompactEDSArray {
                address,
                name,
                object_type,
                data_type,
                access_mode,
                default,
                pdo_mappable,
                low_limit,
                high_limit,
                refuse_write_on_download,
                refuse_read_on_scan,
            } => EDSObject::CompactEDSArray {
                address: address.clone(),
                name: name.clone(),
                object_type: object_type.clone(),
                data_type: data_type.clone(),
                access_mode: access_mode.clone(),
                default: clone_eds_value(default),
                pdo_mappable: pdo_mappable.clone(),
                low_limit: clone_eds_value(low_limit),
                high_limit: clone_eds_value(high_limit),
                refuse_write_on_download: refuse_write_on_download.clone(),
                refuse_read_on_scan: refuse_read_on_scan.clone(),
            },
            EDSObject::EDSArray {
                address,
                name,
                object_type,
                sub_number,
                entries,
                refuse_write_on_download,
                refuse_read_on_scan,
            } => EDSObject::EDSArray {
                address: address.clone(),
                name: name.clone(),
                object_type: object_type.clone(),
                sub_number: sub_number.clone(),
                entries: entries.clone(),
                refuse_write_on_download: refuse_write_on_download.clone(),
                refuse_read_on_scan: refuse_read_on_scan.clone(),
            },
            EDSObject::EDSDomain {
                address,
                name,
                object_type,
                data_type,
                access_mode,
                default,
                refuse_write_on_download,
                refuse_read_on_scan,
            } => EDSObject::EDSDomain {
                address: address.clone(),
                name: name.clone(),
                object_type: object_type.clone(),
                data_type: data_type.clone(),
                access_mode: access_mode.clone(),
                default: clone_eds_value(default),
                refuse_write_on_download: refuse_write_on_download.clone(),
                refuse_read_on_scan: refuse_read_on_scan.clone(),
            },
            EDSObject::EDSNull {
                address,
                name,
                object_type,
            } => EDSObject::EDSNull {
                address: address.clone(),
                name: name.clone(),
                object_type: object_type.clone(),
            },
            EDSObject::EDSVariable {
                address,
                name,
                object_type,
                data_type,
                access_mode,
                default,
                pdo_mappable,
                low_limit,
                high_limit,
                refuse_write_on_download,
                refuse_read_on_scan,
            } => EDSObject::EDSVariable {
                address: address.clone(),
                name: name.clone(),
                object_type: object_type.clone(),
                data_type: data_type.clone(),
                access_mode: access_mode.clone(),
                default: clone_eds_value(default),
                pdo_mappable: pdo_mappable.clone(),
                low_limit: clone_eds_value(low_limit),
                high_limit: clone_eds_value(high_limit),
                refuse_write_on_download: refuse_write_on_download.clone(),
                refuse_read_on_scan: refuse_read_on_scan.clone(),
            },
        }
    }
}

impl EDSObject {
    pub fn get_address(&self) -> &Address {
        match &self {
            EDSObject::CompactEDSArray {
                address,
                name: _,
                object_type: _,
                data_type: _,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => address,
            EDSObject::EDSArray {
                address,
                name: _,
                object_type: _,
                sub_number: _,
                entries: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => address,
            EDSObject::EDSDomain {
                address,
                name: _,
                object_type: _,
                data_type: _,
                access_mode: _,
                default: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => address,
            EDSObject::EDSNull {
                address,
                name: _,
                object_type: _,
            } => address,
            EDSObject::EDSVariable {
                address,
                name: _,
                object_type: _,
                data_type: _,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => address,
        }
    }

    pub fn get_obj_type(&self) -> &ObjectType {
        match &self {
            EDSObject::CompactEDSArray {
                address: _,
                name: _,
                object_type,
                data_type: _,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => object_type,
            EDSObject::EDSArray {
                address: _,
                name: _,
                object_type,
                sub_number: _,
                entries: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => object_type,
            EDSObject::EDSDomain {
                address: _,
                name: _,
                object_type,
                data_type: _,
                access_mode: _,
                default: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => object_type,
            EDSObject::EDSNull {
                address: _,
                name: _,
                object_type,
            } => object_type,
            EDSObject::EDSVariable {
                address: _,
                name: _,
                object_type,
                data_type: _,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => object_type,
        }
    }

    pub fn get_data_type(&self) -> Option<&DataType> {
        match &self {
            EDSObject::EDSVariable {
                address: _,
                name: _,
                object_type: _,
                data_type,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => Some(data_type),
            EDSObject::EDSDomain {
                address: _,
                name: _,
                object_type: _,
                data_type,
                access_mode: _,
                default: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => Some(data_type),
            EDSObject::CompactEDSArray {
                address: _,
                name: _,
                object_type: _,
                data_type,
                access_mode: _,
                default: _,
                pdo_mappable: _,
                low_limit: _,
                high_limit: _,
                refuse_write_on_download: _,
                refuse_read_on_scan: _,
            } => Some(data_type),
            _ => None,
        }
    }

    fn make_addr_string(addr: &Address, with_subindex: bool) -> String {
        if !with_subindex {
            format!("{:x}", addr.index)
        } else {
            format!("{:x}sub{:x}", addr.index, addr.subindex)
        }
    }

    fn parse_access_mode(
        obj: &StructuredFileObject,
        addr: &Address,
    ) -> Result<AccessMode, ParseError> {
        let val = parse_required_str(obj, "AccessType")?;
        AccessMode::parse(val).ok_or(ParseError::InvalidAccessMode {
            address: addr.clone(),
            access_mode: val.clone(),
        })
    }

    fn parse_data_type(obj: &StructuredFileObject, addr: &Address) -> Result<DataType, ParseError> {
        let val = parse_required_uint::<u16>(obj, "DataType")?;
        DataType::parse(val).ok_or(ParseError::InvalidDataType {
            address: addr.clone(),
            data_type: val,
        })
    }

    fn parse_obj_flags(obj: &StructuredFileObject) -> Result<(bool, bool), ParseError> {
        let val =
            parse_required_uint::<u32>(obj, "ObjFlags").or_else(|e| match &e {
                ParseError::MissingRequiredObject { object, section: _ } => {
                    if object == "ObjFlags" { Ok(0) } else { Err(e) }
                }
                _ => Err(e),
            })?;
        let refuse_write_on_download = val & 0b1 != 0;
        let refuse_read_on_scan = val & 0b10 != 0;
        return Ok((refuse_write_on_download, refuse_read_on_scan));
    }

    fn parse_sub_addr(sfile: &StructuredFile, addr: &Address) -> Result<EDSObject, ParseError> {
        let section_name = Self::make_addr_string(addr, true);
        let obj = sfile
            .get_object(&section_name)
            .ok_or(ParseError::MissingRequiredSection {
                section: section_name.clone(),
            })?;
        let obj_type_int = parse_required_uint(obj, "ObjectType")?;
        let obj_type = ObjectType::parse(obj_type_int).ok_or(ParseError::InvalidObjectType {
            address: addr.clone(),
            obj_type: obj_type_int,
        })?;
        let compact = parse_required_bool(obj, "CompactSubObj").unwrap_or(false);
        match obj_type {
            ObjectType::Variable | ObjectType::Deftype => {
                Self::parse_var(obj, obj_type, addr.clone())
            }
            ObjectType::Domain => Self::parse_domain(obj, obj_type, addr.clone()),
            ObjectType::Array | ObjectType::Record | ObjectType::Defstruct => {
                if !compact {
                    Self::parse_array(sfile, obj, obj_type, addr.clone())
                } else {
                    Self::parse_compact_array(obj, obj_type, addr.clone())
                }
            }
            ObjectType::Null => Self::parse_null(obj, obj_type, addr.clone()),
        }
    }

    fn parse_data(
        obj: &StructuredFileObject,
        data_type: &DataType,
        name: &str,
    ) -> Result<Option<Box<dyn EDSValue>>, ParseError> {
        if obj.get_value(name).is_none() {
            return Ok(None);
        }
        match data_type {
            DataType::Boolean => Ok(Some(Box::new(parse_required_bool(obj, name)?))),
            DataType::UInt8 => Ok(Some(Box::new(parse_required_uint::<u8>(obj, name)?))),
            DataType::UInt16 => Ok(Some(Box::new(parse_required_uint::<u16>(obj, name)?))),
            DataType::UInt32 => Ok(Some(Box::new(parse_required_uint::<u32>(obj, name)?))),
            DataType::UInt64 => Ok(Some(Box::new(parse_required_uint::<u64>(obj, name)?))),
            DataType::OctettString | DataType::Domain => {
                Ok(Some(Box::new(parse_required_hex_data(obj, name)?)))
            }
            DataType::UnicodeString => Ok(Some(Box::new(parse_required_str(obj, name)?.clone()))),
            DataType::VisibleString => {
                let val = parse_required_str(obj, name)?.clone();
                if !val.is_ascii() {
                    Err(ParseError::InvalidValueFormat {
                        object: name.to_string(),
                        section: obj.get_name().clone(),
                    })
                } else {
                    Ok(Some(Box::new(val)))
                }
            }
            DataType::Int8 => Ok(Some(Box::new(parse_required_uint::<u8>(obj, name)? as i8))),
            DataType::Int16 => Ok(Some(
                Box::new(parse_required_uint::<u16>(obj, name)? as i16),
            )),
            DataType::Int32 => Ok(Some(
                Box::new(parse_required_uint::<u32>(obj, name)? as i32),
            )),
            DataType::Int64 => Ok(Some(
                Box::new(parse_required_uint::<u64>(obj, name)? as i64),
            )),
            DataType::Real32 => Ok(Some(Box::new(parse_required_float(obj, name)?))),
            DataType::Real64 => Ok(Some(Box::new(parse_required_double(obj, name)?))),
        }
    }

    fn parse_var(
        obj: &StructuredFileObject,
        obj_type: ObjectType,
        addr: Address,
    ) -> Result<EDSObject, ParseError> {
        let name = parse_required_str(obj, "ParameterName")?;
        let access_mode = Self::parse_access_mode(obj, &addr)?;
        let (refuse_write_on_download, refuse_read_on_scan) = Self::parse_obj_flags(obj)?;
        let data_type = Self::parse_data_type(obj, &addr)?;
        let default = Self::parse_data(obj, &data_type, "Default")?;
        let low_limit = Self::parse_data(obj, &data_type, "LowLimit")?;
        let high_limit = Self::parse_data(obj, &data_type, "HighLimit")?;
        let pdo_mappable = parse_required_bool(obj, "PDOMapping").or_else(|e| match &e {
            ParseError::MissingRequiredObject {
                object: _,
                section: _,
            } => Ok(false),
            _ => Err(e),
        })?;

        if !access_mode.is_valid(pdo_mappable) {
            return Err(ParseError::PDOMappableNotSupportedForAccessType {
                addr: addr,
                access_type: access_mode,
            });
        }
        match data_type {
            DataType::Boolean
            | DataType::OctettString
            | DataType::UnicodeString
            | DataType::Domain
            | DataType::VisibleString => {
                if low_limit.is_some() || high_limit.is_some() {
                    return Err(ParseError::ObjectTypeDoesNotSupportLimits {
                        addr: addr,
                        object_type: obj_type,
                    });
                }
            }
            _ => {}
        }

        Ok(EDSObject::EDSVariable {
            address: addr,
            name: name.clone(),
            object_type: obj_type,
            data_type: data_type,
            access_mode: access_mode,
            default: default,
            pdo_mappable: pdo_mappable,
            low_limit: low_limit,
            high_limit: high_limit,
            refuse_write_on_download: refuse_write_on_download,
            refuse_read_on_scan: refuse_read_on_scan,
        })
    }

    fn parse_domain(
        obj: &StructuredFileObject,
        obj_type: ObjectType,
        addr: Address,
    ) -> Result<EDSObject, ParseError> {
        let name = parse_required_str(obj, "ParameterName")?;
        let access_mode = Self::parse_access_mode(obj, &addr)?;
        let (refuse_write_on_download, refuse_read_on_scan) = Self::parse_obj_flags(obj)?;
        let data_type = Self::parse_data_type(obj, &addr).or_else(|e| match &e {
            ParseError::MissingRequiredObject { object, section: _ } => {
                if object == "DataType" {
                    Ok(DataType::Domain)
                } else {
                    Err(e)
                }
            }
            _ => Err(e),
        })?;
        let default = Self::parse_data(obj, &data_type, "Default")?;
        Ok(EDSObject::EDSDomain {
            address: addr,
            name: name.clone(),
            object_type: obj_type,
            data_type: data_type,
            access_mode: access_mode,
            default: default,
            refuse_write_on_download: refuse_write_on_download,
            refuse_read_on_scan: refuse_read_on_scan,
        })
    }

    fn parse_array(
        sfile: &StructuredFile,
        obj: &StructuredFileObject,
        obj_type: ObjectType,
        addr: Address,
    ) -> Result<EDSObject, ParseError> {
        let name = parse_required_str(obj, "ParameterName")?;
        let (refuse_write_on_download, refuse_read_on_scan) = Self::parse_obj_flags(obj)?;
        let sub_number = parse_required_uint(obj, "SubNumber")?;
        let mut entries = Vec::new();
        for i in 0..sub_number {
            let subaddr = Address::new(addr.index, i);
            let section_name = Self::make_addr_string(&subaddr, true);
            let sub = Self::parse_sub_addr(sfile, &subaddr);

            //Ignore not found subnumbers
            match sub {
                Err(ParseError::MissingRequiredSection { section }) => {
                    if section_name != section {
                        return Err(ParseError::MissingRequiredSection { section: section });
                    }
                }
                Err(_) => {
                    return sub;
                }
                Ok(val) => entries.push(val),
            }
        }
        //No nested lists
        for e in &entries {
            if matches!(
                e.get_obj_type(),
                ObjectType::Array | ObjectType::Defstruct | ObjectType::Record
            ) {
                return Err(ParseError::NestedListsUnsupported {
                    addr: e.get_address().clone(),
                });
            }
        }
        //Assert array properties
        if obj_type == ObjectType::Array {
            if entries.len() > 1 {
                let first_entry = &entries[1];
                for e in &entries {
                    if e.get_address().subindex == 0 {
                        continue;
                    }
                    if first_entry.get_obj_type() != e.get_obj_type() {
                        return Err(ParseError::InconsistentObjectDefinition {
                            addr: e.get_address().clone(),
                        });
                    }
                    if first_entry.get_data_type() != e.get_data_type() {
                        return Err(ParseError::InconsistentObjectDefinition {
                            addr: e.get_address().clone(),
                        });
                    }
                }
            }
        }
        Ok(EDSObject::EDSArray {
            address: addr,
            name: name.clone(),
            object_type: obj_type,
            entries: entries,
            sub_number: sub_number,
            refuse_write_on_download: refuse_write_on_download,
            refuse_read_on_scan: refuse_read_on_scan,
        })
    }

    fn parse_compact_array(
        obj: &StructuredFileObject,
        obj_type: ObjectType,
        addr: Address,
    ) -> Result<EDSObject, ParseError> {
        let name = parse_required_str(obj, "ParameterName")?;
        let access_mode = Self::parse_access_mode(obj, &addr)?;
        let (refuse_write_on_download, refuse_read_on_scan) = Self::parse_obj_flags(obj)?;
        let data_type = Self::parse_data_type(obj, &addr)?;
        let default = Self::parse_data(obj, &data_type, "Default")?;
        let low_limit = Self::parse_data(obj, &data_type, "LowLimit")?;
        let high_limit = Self::parse_data(obj, &data_type, "HighLimit")?;
        let pdo_mappable = parse_required_bool(obj, "PDOMapping").or_else(|e| match &e {
            ParseError::MissingRequiredObject {
                object: _,
                section: _,
            } => Ok(false),
            _ => Err(e),
        })?;

        if !access_mode.is_valid(pdo_mappable) {
            return Err(ParseError::PDOMappableNotSupportedForAccessType {
                addr: addr,
                access_type: access_mode,
            });
        }
        match data_type {
            DataType::Boolean
            | DataType::OctettString
            | DataType::UnicodeString
            | DataType::Domain
            | DataType::VisibleString => {
                if low_limit.is_some() || high_limit.is_some() {
                    return Err(ParseError::ObjectTypeDoesNotSupportLimits {
                        addr: addr,
                        object_type: obj_type,
                    });
                }
            }
            _ => {}
        }
        Ok(EDSObject::CompactEDSArray {
            address: addr,
            name: name.clone(),
            object_type: obj_type,
            data_type: data_type,
            access_mode: access_mode,
            default: default,
            pdo_mappable: pdo_mappable,
            low_limit: low_limit,
            high_limit: high_limit,
            refuse_write_on_download: refuse_write_on_download,
            refuse_read_on_scan: refuse_read_on_scan,
        })
    }

    fn parse_null(
        obj: &StructuredFileObject,
        obj_type: ObjectType,
        addr: Address,
    ) -> Result<EDSObject, ParseError> {
        let name = parse_required_str(obj, "ParameterName")?;
        Ok(EDSObject::EDSNull {
            address: addr,
            name: name.to_string(),
            object_type: obj_type,
        })
    }

    pub fn parse(sfile: &StructuredFile, addr: &Address) -> Result<EDSObject, ParseError> {
        let section_name = Self::make_addr_string(addr, false);
        let obj = sfile
            .get_object(&section_name)
            .ok_or(ParseError::MissingRequiredSection {
                section: section_name.clone(),
            })?;
        let obj_type_int = parse_required_uint(obj, "ObjectType")?;
        let obj_type = ObjectType::parse(obj_type_int).ok_or(ParseError::InvalidObjectType {
            address: addr.clone(),
            obj_type: obj_type_int,
        })?;
        let compact = parse_required_bool(obj, "CompactSubObj").unwrap_or(false);
        match obj_type {
            ObjectType::Variable | ObjectType::Deftype => {
                Self::parse_var(obj, obj_type, addr.clone())
            }
            ObjectType::Domain => Self::parse_domain(obj, obj_type, addr.clone()),
            ObjectType::Array | ObjectType::Record | ObjectType::Defstruct => {
                if !compact {
                    Self::parse_array(sfile, obj, obj_type, addr.clone())
                } else {
                    Self::parse_compact_array(obj, obj_type, addr.clone())
                }
            }
            ObjectType::Null => Self::parse_null(obj, obj_type, addr.clone()),
        }
    }
}
