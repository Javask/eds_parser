pub use error::*;

#[cfg(test)]
mod tests;

mod error;
mod raw_file;
mod structured_file;
mod eds_file;

use raw_file::RawFile;
use structured_file::StructuredFile;
pub use eds_file::EDSFile;


pub fn load_file(filename: &str) -> Result<EDSFile, ParseError> {
    let rfile = RawFile::new(filename)?;
    let sfile = StructuredFile::parse(rfile)?;
    EDSFile::parse(sfile)
}
