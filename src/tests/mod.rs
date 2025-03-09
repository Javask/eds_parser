pub(crate) mod utils;

use crate::load_file;

#[test]
fn test_integration() {
    for file in vec!["res/micro-motor.eds"] {
        let sfile = load_file(file);
        if sfile.as_ref().is_err() {
            println!("Error in file {}: {:?}", file, sfile.as_ref().err());
        }
        assert!(sfile.is_ok());
    }
}
