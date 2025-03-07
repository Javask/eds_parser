use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};

use super::eds_date::EDSDate;
use super::eds_time::EDSTime;
use crate::ParseError;
use crate::structured_file::StructuredFileObject;

pub fn parse_hex_str<T: TryFrom<u64>, const BITS: u8>(string: &str) -> Option<T> {
    if !string.is_ascii() {
        return None;
    }
    let bytes: Result<Vec<u8>, _> = (0..string.len())
        .map(|i| u8::from_str_radix(&string[i..i + 1], (2 as u32).pow(BITS as u32)))
        .collect();
    if bytes.is_err() {
        return None;
    }
    let mut out: u64 = 0;
    for b in bytes.unwrap() {
        out = out << BITS | b as u64;
    }
    T::try_from(out).ok()
}

pub fn parse_required_str<'a>(
    obj: &'a StructuredFileObject,
    name: &str,
) -> Result<&'a String, ParseError> {
    obj.get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })
}

pub fn parse_required_hex_data(
    obj: &StructuredFileObject,
    name: &str,
) -> Result<Vec<u8>, ParseError> {
    let val = obj
        .get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })?;
    if !val.is_ascii() {
        return Err(ParseError::ParseHexError {
            object: name.to_string(),
            section: obj.get_name().clone(),
            value: val.clone(),
        });
    }

    let bytes: Result<Vec<u8>, _> = (0..val.len())
        .map(|i| u8::from_str_radix(&val[i..i + 1], 16))
        .collect();
    bytes.or_else(|e| {
        Err(ParseError::ParseIntError {
            object: name.to_string(),
            section: obj.get_name().clone(),
            err: e,
        })
    })
}

pub fn parse_required_float(obj: &StructuredFileObject, name: &str) -> Result<f32, ParseError> {
    let val = obj
        .get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })?;
    val.parse::<f32>().or_else(|e| {
        Err(ParseError::ParseFloatError {
            section: obj.get_name().to_string(),
            object: name.to_string(),
            err: e,
        })
    })
}
pub fn parse_required_double(obj: &StructuredFileObject, name: &str) -> Result<f64, ParseError> {
    let val = obj
        .get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })?;
    val.parse::<f64>().or_else(|e| {
        Err(ParseError::ParseFloatError {
            section: obj.get_name().to_string(),
            object: name.to_string(),
            err: e,
        })
    })
}

pub fn parse_required_uint<T: FromStr<Err = ParseIntError> + TryFrom<u64>>(
    obj: &StructuredFileObject,
    name: &str,
) -> Result<T, ParseError> {
    let val = obj
        .get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })?;

    if val.starts_with("0x") {
        parse_hex_str::<T, 4>(&val[2..]).ok_or(ParseError::ParseHexError {
            object: name.to_string(),
            section: obj.get_name().clone(),
            value: val[2..].to_string(),
        })
    } else if val.starts_with("0") && val != "0" {
        parse_hex_str::<T, 3>(&val[1..]).ok_or(ParseError::ParseOctalError {
            object: name.to_string(),
            section: obj.get_name().clone(),
            value: val[1..].to_string(),
        })
    } else {
        val.parse::<T>().or_else(|e| {
            Err(ParseError::ParseIntError {
                section: obj.get_name().to_string(),
                object: name.to_string(),
                err: e,
            })
        })
    }
}

pub fn parse_required_bool(obj: &StructuredFileObject, name: &str) -> Result<bool, ParseError> {
    obj.get_value(name)
        .ok_or(ParseError::MissingRequiredObject {
            object: name.to_string(),
            section: obj.get_name().clone(),
        })?
        .parse::<u8>()
        .or_else(|e| {
            Err(ParseError::ParseIntError {
                section: obj.get_name().to_string(),
                object: name.to_string(),
                err: e,
            })
        })
        .and_then(|v| {
            if v == 1 {
                Ok(true)
            } else if v == 0 {
                Ok(false)
            } else {
                Err(ParseError::InvalidValueFormat {
                    object: obj.get_name().to_string(),
                    section: name.to_string(),
                })
            }
        })
}

pub fn parse_date_time_combo(
    obj: &StructuredFileObject,
    date_name: &str,
    time_name: &str,
) -> Result<DateTime<Utc>, ParseError> {
    let date = EDSDate::parse(obj, date_name)?;
    let time = EDSTime::parse(obj, time_name)?;
    let res = Utc.with_ymd_and_hms(
        date.year,
        date.month as u32,
        date.day as u32,
        time.hour as u32,
        time.minute as u32,
        0,
    );
    match res {
        chrono::offset::LocalResult::Single(s) => Ok(s),
        _ => Err(ParseError::ParseTimeError {
            date_obj: date_name.to_string(),
            time_obj: time_name.to_string(),
            section: obj.get_name().to_string(),
        }),
    }
}
