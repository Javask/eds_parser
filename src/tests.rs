mod raw_file;
mod structured_file;
mod utils;

use crate::load_file;


#[test]
fn test_integration(){
    let sfile = load_file("res/test.eds");
    if sfile.as_ref().is_err(){
        println!("Error: {:?}",sfile.as_ref().err());
    }
    assert!(sfile.is_ok());
}