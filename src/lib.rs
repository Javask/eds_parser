pub use error::*;

#[cfg(test)]
mod tests;

mod eds_file;
mod error;
mod raw_file;
mod structured_file;

pub use eds_file::*;
use raw_file::RawFile;
use structured_file::StructuredFile;

pub fn load_file(filename: &str) -> Result<EDSFile, ParseError> {
    let rfile = RawFile::new(filename)?;
    let sfile = StructuredFile::parse(rfile)?;
    EDSFile::parse(sfile)
}
