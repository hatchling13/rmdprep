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

pub fn read_line(file_name: &str, contents: &mut Vec<(usize, String)>) -> std::result::Result<usize, std::io::Error> {
    let file = File::open(file_name);
    
    match file {
        Ok(_) => {
            let file_buffer = BufReader::new(file.unwrap());

            let mut line_num = 1;
            
            for line in file_buffer.lines() {
                contents.push((line_num, line.unwrap()));

                line_num += 1;
            }

            return Ok(line_num);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn write_file(input: &String) -> std::io::Result<()> {
    let mut file = File::create("result.md")?;

    if let Err(e) = file.write_all(input.as_bytes()) {
        return Err(e);
    }

    Ok(())
}