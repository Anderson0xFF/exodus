use std::{fs::read_to_string, io::Error};

pub fn open_file(path: &str) -> Result<String, Error> {
    match read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}
