use crate::error::ParseError;
use crate::raw_file::RawFile;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

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

    pub fn get_value(&self, name: &str) -> Option<&String> {
        let lower_name = name.to_lowercase();
        for val in self.values.iter() {
            if val.0.to_lowercase() == lower_name {
                return Some(val.1);
            }
        }
        None
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
        let val = &line[equal_pos.unwrap() + 1..];
        if key.is_empty() || val.is_empty() {
            return None;
        }

        Some((key, val))
    }
    
    #[cfg(test)]
    pub fn get_objects(&self) -> &Vec<StructuredFileObject>{
        &self.objects
    }

    pub fn get_object(&self, name: &str) -> Option<&StructuredFileObject> {
        let lower_name = name.to_lowercase();
        for obj in &self.objects{
            if obj.get_name().to_lowercase() == lower_name{
                return Some(obj);
            }
        }
        None
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
                        section: obj.get_name().clone(),
                        object: key,
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
