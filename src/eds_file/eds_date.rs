use regex::Regex;

use crate::{ParseError, structured_file::StructuredFileObject};

use super::utils::parse_required_str;

#[derive(Debug, PartialEq)]
pub struct EDSDate {
   pub year: i32,
   pub month: u8,
   pub day: u8,
}

impl EDSDate {
    pub fn parse(obj: &StructuredFileObject, name: &str) -> Result<EDSDate, ParseError> {
        let value_str = parse_required_str(obj, name)?;
        let date_regex = Regex::new("([0-1][0-9])-([0-3][0-9])-([0-9][0-9][0-9][0-9])")
            .expect("Failed to compile date regex!");
        let Some(captures) = date_regex.captures(value_str) else {
            return Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().to_string(),
            });
        };
        let months = captures[1]
            .parse::<u8>()
            .or(Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().to_string(),
            }))?;
        let days = captures[2]
            .parse::<u8>()
            .or(Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().to_string(),
            }))?;
        let years = captures[3]
            .parse::<i32>()
            .or(Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().to_string(),
            }))?;
        //TODO find out which months have how many days
        if days > 31 || months > 12 || months == 0 || days == 0{
            return Err(ParseError::InvalidValueFormat {
                object: name.to_string(),
                section: obj.get_name().to_string(),
            });
        }
        Ok(EDSDate {
            year: years,
            month: months,
            day: days,
        })
    }
}
