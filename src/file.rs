use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &str, contents: &mut String) -> std::result::Result<usize, std::io::Error> {
    let file = File::open(file_name);
    
    match file {
        Ok(_) => {
            let mut file_buffer = BufReader::new(file.unwrap());
            let result = file_buffer.read_to_string(contents);
        
            result
        }
        Err(e) => {
            return Err(e);
        }
    }
}