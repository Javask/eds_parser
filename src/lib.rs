use std::{collections::HashMap, fs::File, io::Read};

#[cfg(test)]
mod tests;

mod error;
pub use error::ParseError;

pub struct RawFile {
    lines: Vec<String>,
}

impl RawFile {
    fn read_lines(file: &mut File) -> Result<Vec<String>, std::io::Error> {
        let mut result = Vec::new();
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        for line in data.lines() {
            result.push(line.to_string())
        }
        return Ok(result);
    }

    pub fn new(filename: &str) -> Result<RawFile, ParseError> {
        let mut file = File::open(filename)?;
        Self::new_from_file(&mut file)
    }

    pub fn new_from_file(file: &mut File) -> Result<RawFile, ParseError> {
        let lines = Self::read_lines(file)?;
        Ok(RawFile { lines: lines })
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }
}

pub struct StructuredFileObject {
    name: String,
    values: HashMap<String, String>,
}

impl StructuredFileObject {
    pub fn new(name: String) -> Self {
        StructuredFileObject {
            name: name,
            values: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_values(&self) -> &HashMap<String, String> {
        &self.values
    }

    pub fn get_values_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.values
    }
}

pub struct StructuredFile {
    objects: Vec<StructuredFileObject>,
}

impl StructuredFile {
    fn parse_section_line(line: &str) -> Option<&str> {
        if !line.starts_with('[') {
            return None;
        }
        Some(line.trim_start_matches('[').trim_end_matches(']'))
    }

    fn parse_value_line(line: &str) -> Option<(&str, &str)> {
        let equal_pos = line.find('=');
        if equal_pos.is_none() {
            return None;
        }

        let key = &line[0..equal_pos.unwrap()];
        let val = &line[equal_pos.unwrap()+1..];
        if key.is_empty() || val.is_empty() {
            return None;
        }

        Some((key, val))
    }

    pub fn get_objects(&self) -> &Vec<StructuredFileObject> {
        &self.objects
    }

    pub fn parse(raw: RawFile) -> Result<StructuredFile, ParseError> {
        let lines = raw.get_lines();
        let mut out = StructuredFile {
            objects: Vec::new(),
        };
        let mut obj = StructuredFileObject::new("".to_string());

        for line in lines {
            let s = line.trim();
            //Remove empty lines and comments
            if s.is_empty() || s.starts_with(';') {
                continue;
            }
            let section_name = Self::parse_section_line(line);
            if section_name.is_some() {
                if !obj.get_name().is_empty() {
                    out.objects.push(obj);
                }
                obj = StructuredFileObject::new(section_name.unwrap().to_string());
                continue;
            }
            let value = Self::parse_value_line(line);
            if value.is_some() {
                let key = value.unwrap().0.to_string();
                let val = value.unwrap().1.to_string();
                if obj.get_name().is_empty() {
                    return Err(ParseError::NoObjectForValue { value: key });
                }

                if obj.get_values().contains_key(value.unwrap().0) {
                    return Err(ParseError::DoubleValueDefinition {
                        object: obj.get_name().clone(),
                        value: key,
                    });
                }
                obj.get_values_mut().insert(key, val);
                continue;
            }
            return Err(ParseError::InvalidFormatting {
                line: s.to_string(),
            });
        }
        if !obj.get_name().is_empty() {
            out.objects.push(obj);
        }
        Ok(out)
    }
}
