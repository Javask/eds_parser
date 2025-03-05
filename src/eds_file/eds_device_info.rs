use crate::{ParseError, structured_file::StructuredFileObject};

use super::utils::{parse_required_bool, parse_required_str, parse_required_uint};

pub struct EDSDeviceInfo {
    pub vendor_name: String,
    pub vendor_number: u32, //0x1000.1
    pub product_name: String,
    pub product_number: u32,  //0x1000.2
    pub revision_number: u32, //0x1000.3
    pub order_code: String,
    pub baudrate_10khz: bool,
    pub baudrate_20khz: bool,
    pub baudrate_50khz: bool,
    pub baudrate_125khz: bool,
    pub baudrate_250khz: bool,
    pub baudrate_500khz: bool,
    pub baudrate_800khz: bool,
    pub baudrate_1000khz: bool,
    pub simple_bootup_master: bool,
    pub simple_bootup_slave: bool,
    pub granularity: u8,
    pub dynamic_channel_supported: bool,
    pub group_messaging: bool,
    pub nr_rpdo: u16,
    pub nr_tpdo: u16,
    pub lss_supported: bool,
}

impl EDSDeviceInfo {
    pub fn parse(obj: &StructuredFileObject) -> Result<EDSDeviceInfo, ParseError> {
        let vendor_name = parse_required_str(obj, "VendorName")?;
        let vendor_number = parse_required_uint(obj, "VendorNumber")?;
        let product_name = parse_required_str(obj, "ProductName")?;
        let product_number = parse_required_uint(obj, "ProductNumber")?;
        let revision_number = parse_required_uint(obj, "RevisionNumber")?;
        let order_code = parse_required_str(obj, "OrderCode")?;
        let baudrate_10 = parse_required_bool(obj, "BaudRate_10")?;
        let baudrate_20 = parse_required_bool(obj, "BaudRate_20")?;
        let baudrate_50 = parse_required_bool(obj, "BaudRate_50")?;
        let baudrate_125 = parse_required_bool(obj, "BaudRate_125")?;
        let baudrate_250 = parse_required_bool(obj, "BaudRate_250")?;
        let baudrate_500 = parse_required_bool(obj, "BaudRate_500")?;
        let baudrate_800 = parse_required_bool(obj, "BaudRate_800")?;
        let baudrate_1000 = parse_required_bool(obj, "BaudRate_1000")?;
        let simple_master = parse_required_bool(obj, "SimpleBootUpMaster")?;
        let simple_slave = parse_required_bool(obj, "SimpleBootUpSlave")?;
        let granularity = parse_required_uint(obj, "Granularity")?;
        if granularity < 1 || granularity > 64 {
            return Err(ParseError::InvalidValueFormat {
                object: "Granularity".to_string(),
                section: obj.get_name().to_string(),
            });
        }
        let dynamic_channels = parse_required_bool(obj, "DynamicChannelsSupported")?;
        let group_messaging = parse_required_bool(obj, "GroupMessaging")?;
        let nr_rpdo = parse_required_uint(obj, "NrOfRXPDO")?;
        let nr_tpdo = parse_required_uint(obj, "NrOfTXPDO")?;
        let lss_supported = parse_required_bool(obj, "LSS_Supported")?;
        Ok(EDSDeviceInfo {
            vendor_name: vendor_name.clone(),
            vendor_number: vendor_number,
            product_name: product_name.clone(),
            product_number: product_number,
            revision_number: revision_number,
            order_code: order_code.clone(),
            baudrate_10khz: baudrate_10,
            baudrate_20khz: baudrate_20,
            baudrate_50khz: baudrate_50,
            baudrate_125khz: baudrate_125,
            baudrate_250khz: baudrate_250,
            baudrate_500khz: baudrate_500,
            baudrate_800khz: baudrate_800,
            baudrate_1000khz: baudrate_1000,
            simple_bootup_master: simple_master,
            simple_bootup_slave: simple_slave,
            granularity: granularity,
            dynamic_channel_supported: dynamic_channels,
            group_messaging: group_messaging,
            nr_rpdo: nr_rpdo,
            nr_tpdo: nr_tpdo,
            lss_supported: lss_supported,
        })
    }
}
