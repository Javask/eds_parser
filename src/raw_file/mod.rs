use std::{fs::File, io::Read};

use crate::error::ParseError;

#[cfg(test)]
mod tests;

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