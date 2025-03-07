use crate::ParseError;
use crate::eds_file::EDSFile;
use crate::raw_file::RawFile;
use crate::structured_file::StructuredFile;
use crate::tests::utils::*;

#[test]
fn test_phase_3_success() {
    let data = vec![
        "[FileInfo]",
        "FileName=test.eds",
        "FileVersion=1",
        "FileRevision=4",
        "EDSVersion=4.0",
        "Description=aaaaaaaaa",
        "CreationTime=04:09PM",
        "CreationDate=08-06-2012",
        "CreatedBy=Peabody",
        "ModificationTime=02:46PM",
        "ModificationDate=04-25-2014",
        "ModifiedBy=Peabody",
        "",
        "[DeviceInfo]",
        "VendorName=Aperture Science",
        "VendorNumber=0x286",
        "ProductName=ED201",
        "ProductNumber=0x488",
        "RevisionNumber=0x00030012",
        "OrderCode=ED201",
        "BaudRate_10=0",
        "BaudRate_20=1",
        "BaudRate_50=1",
        "BaudRate_125=1",
        "BaudRate_250=1",
        "BaudRate_500=1",
        "BaudRate_800=1",
        "BaudRate_1000=1",
        "DynamicChannelsSupported=0",
        "GroupMessaging=0",
        "LSS_Supported=0",
        "Granularity=8",
        "SimpleBootUpSlave=1",
        "SimpleBootUpMaster=0",
        "NrOfRXPDO=5",
        "NrOfTXPDO=5",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let _efile = EDSFile::parse(sfile).expect("Failed to parse eds file!");
}


#[test]
fn test_phase_3_invalid_granularity() {
    let data = vec![
        "[FileInfo]",
        "FileName=test.eds",
        "FileVersion=1",
        "FileRevision=4",
        "EDSVersion=4.0",
        "Description=aaaaaaaaa",
        "CreationTime=04:09PM",
        "CreationDate=08-06-2012",
        "CreatedBy=Peabody",
        "ModificationTime=02:46PM",
        "ModificationDate=04-25-2014",
        "ModifiedBy=Peabody",
        "",
        "[DeviceInfo]",
        "VendorName=Aperture Science",
        "VendorNumber=0x286",
        "ProductName=ED201",
        "ProductNumber=0x488",
        "RevisionNumber=0x00030012",
        "OrderCode=ED201",
        "BaudRate_10=0",
        "BaudRate_20=1",
        "BaudRate_50=1",
        "BaudRate_125=1",
        "BaudRate_250=1",
        "BaudRate_500=1",
        "BaudRate_800=1",
        "BaudRate_1000=1",
        "DynamicChannelsSupported=0",
        "GroupMessaging=0",
        "LSS_Supported=0",
        "Granularity=100",
        "SimpleBootUpSlave=1",
        "SimpleBootUpMaster=0",
        "NrOfRXPDO=5",
        "NrOfTXPDO=5",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let efile = EDSFile::parse(sfile);
    assert!(efile.is_err());
    let err = efile.unwrap_err();
    let ParseError::InvalidValueFormat { object, section } = err else{
        panic!();
    };
    assert_eq!(object,"Granularity");
    assert_eq!(section,"DeviceInfo");

}


#[test]
fn test_phase_3_incomplete_file_info() {
    let data = vec![
        "[FileInfo]",
        "FileName=test.eds",
        "FileRevision=4",
        "EDSVersion=4.0",
        "Description=aaaaaaaaa",
        "CreationTime=04:09PM",
        "CreationDate=08-06-2012",
        "CreatedBy=Peabody",
        "ModificationTime=02:46PM",
        "ModificationDate=04-25-2014",
        "ModifiedBy=Peabody",
        "",
        "[DeviceInfo]",
        "VendorName=Aperture Science",
        "VendorNumber=0x286",
        "ProductName=ED201",
        "ProductNumber=0x488",
        "RevisionNumber=0x00030012",
        "OrderCode=ED201",
        "BaudRate_10=0",
        "BaudRate_20=1",
        "BaudRate_50=1",
        "BaudRate_125=1",
        "BaudRate_250=1",
        "BaudRate_500=1",
        "BaudRate_800=1",
        "BaudRate_1000=1",
        "DynamicChannelsSupported=0",
        "GroupMessaging=0",
        "LSS_Supported=0",
        "Granularity=8",
        "SimpleBootUpSlave=1",
        "SimpleBootUpMaster=0",
        "NrOfRXPDO=5",
        "NrOfTXPDO=5",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let efile = EDSFile::parse(sfile);
    match efile {
        Err(ParseError::MissingRequiredObject { object, section }) => {
            assert_eq!(section, "FileInfo");
            assert_eq!(object, "FileVersion");
        }
        _ => panic!(),
    }
}

#[test]
fn test_phase_3_no_file_info() {
    let data = vec![
        "[DeviceInfo]",
        "VendorName=Aperture Science",
        "VendorNumber=0x286",
        "ProductName=ED201",
        "ProductNumber=0x488",
        "RevisionNumber=0x00030012",
        "OrderCode=ED201",
        "BaudRate_10=0",
        "BaudRate_20=1",
        "BaudRate_50=1",
        "BaudRate_125=1",
        "BaudRate_250=1",
        "BaudRate_500=1",
        "BaudRate_800=1",
        "BaudRate_1000=1",
        "DynamicChannelsSupported=0",
        "GroupMessaging=0",
        "LSS_Supported=0",
        "Granularity=8",
        "SimpleBootUpSlave=1",
        "SimpleBootUpMaster=0",
        "NrOfRXPDO=5",
        "NrOfTXPDO=5",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let efile = EDSFile::parse(sfile);
    match efile {
        Err(ParseError::MissingRequiredSection { section }) => assert_eq!(section, "FileInfo"),
        _ => panic!(),
    }
}

#[test]
fn test_phase_3_no_device_info() {
    let data = vec![
        "[FileInfo]",
        "FileName=test.eds",
        "FileRevision=4",
        "FileVersion=1",
        "EDSVersion=4.0",
        "Description=aaaaaaaaa",
        "CreationTime=04:09PM",
        "CreationDate=08-06-2012",
        "CreatedBy=Peabody",
        "ModificationTime=02:46PM",
        "ModificationDate=04-25-2014",
        "ModifiedBy=Peabody",
        "",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let efile = EDSFile::parse(sfile);
    println!("Res: {:?}", efile);
    match efile {
        Err(ParseError::MissingRequiredSection { section }) => assert_eq!(section, "DeviceInfo"),
        _ => panic!(),
    }
}

#[test]
fn test_phase_3_partial_device_info() {
    let data = vec![
        "[FileInfo]",
        "FileName=test.eds",
        "FileRevision=4",
        "FileVersion=1",
        "EDSVersion=4.0",
        "Description=aaaaaaaaa",
        "CreationTime=04:09PM",
        "CreationDate=08-06-2012",
        "CreatedBy=Peabody",
        "ModificationTime=02:46PM",
        "ModificationDate=04-25-2014",
        "ModifiedBy=Peabody",
        "",
        "[DeviceInfo]",
        "VendorName=Aperture Science",
        "VendorNumber=0x286",
        "ProductName=ED201",
        "ProductNumber=0x488",
        "OrderCode=ED201",
        "BaudRate_10=0",
        "BaudRate_20=1",
        "BaudRate_50=1",
        "BaudRate_125=1",
        "BaudRate_250=1",
        "BaudRate_500=1",
        "BaudRate_800=1",
        "BaudRate_1000=1",
        "DynamicChannelsSupported=0",
        "GroupMessaging=0",
        "LSS_Supported=0",
        "Granularity=8",
        "SimpleBootUpSlave=1",
        "SimpleBootUpMaster=0",
        "NrOfRXPDO=5",
        "NrOfTXPDO=5",
        "[MandatoryObjects]",
        "SupportedObjects=0",
        "[OptionalObjects]",
        "SupportedObjects=0",
        "[ManufacturerObjects]",
        "SupportedObjects=0",
    ];
    let raw = make_string(&data);
    let mut tmp = make_tmp_file(raw);
    let rfile = RawFile::new_from_file(&mut tmp).expect("Failed to read back lines from file!");
    let sfile = StructuredFile::parse(rfile).expect("Failed to parse correct string!");
    let efile = EDSFile::parse(sfile);
    match efile {
        Err(ParseError::MissingRequiredObject { object, section }) => {
            assert_eq!(section, "DeviceInfo");
            assert_eq!(object, "RevisionNumber");
        }
        _ => panic!(),
    }
}