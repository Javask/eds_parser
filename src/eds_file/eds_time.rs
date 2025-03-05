use super::utils::parse_required_str;
use crate::ParseError;
use crate::structured_file::StructuredFileObject;
use regex::Regex;

pub struct EDSTime {
    pub hour: u8,
    pub minute: u8,
}

impl EDSTime {
    pub fn parse(obj: &StructuredFileObject, name: &str) -> Result<EDSTime, ParseError> {
        let string_val = parse_required_str(obj, name)?;
        let time_regex = Regex::new("([0-1][0-9]):([0-5][0-9]) *(AM|PM)")
            .expect("Failed to compile time regex!");
        let Some(captures) = time_regex.captures(&string_val) else {
            return Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().clone(),
            });
        };

        let mut hours = captures[1]
            .parse::<u8>()
            .or(Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().clone(),
            }))?;
        let minute = captures[2]
            .parse::<u8>()
            .or(Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().clone(),
            }))?;
        let am_pm = &captures[3];
        let is_am: bool;
        if am_pm == "AM" {
            is_am = true;
        } else if am_pm == "PM" {
            is_am = false;
        } else {
            return Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().clone(),
            });
        }
        if hours > 12 || hours == 0 || minute >= 60 {
            return Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().clone(),
            });
        };
        if is_am && hours == 12 {
            hours = 0;
        } else if !is_am && hours != 12 {
            hours += 12;
        }
        Ok(EDSTime {
            hour: hours,
            minute: minute,
        })
    }
}
